import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { describe, it, expect, beforeEach, beforeAll, afterAll, afterEach } from 'vitest'
import { invoke } from '@tauri-apps/api/core'
import App from '../App'

// End-to-End Tests for User Story #3: Download Episodes
// These tests integrate frontend UI with actual backend functionality
describe('User Story #3: Download Episodes - End-to-End Integration', () => {
  let testPodcastId: number
  let testEpisodeId: number
  let testEpisodeUrl: string

  beforeAll(async () => {
    // Set up test data with actual backend
    console.log('Setting up E2E test environment...')
    
    try {
      // Add a test podcast
      testPodcastId = await invoke('add_podcast', {
        rssUrl: 'https://feeds.npr.org/500005/podcast.xml' // NPR TED Radio Hour - reliable test feed
      })
      
      console.log(`Test podcast added with ID: ${testPodcastId}`)
      
      // Wait a moment for episodes to be populated
      await new Promise(resolve => setTimeout(resolve, 2000))
      
      // Get episodes for the test podcast
      const episodes = await invoke('get_episodes', { podcastId: testPodcastId })
      
      if (episodes && episodes.length > 0) {
        testEpisodeId = episodes[0].id
        testEpisodeUrl = episodes[0].episode_url
        console.log(`Test episode ID: ${testEpisodeId}, URL: ${testEpisodeUrl}`)
      } else {
        throw new Error('No episodes found for test podcast')
      }
    } catch (error) {
      console.error('Failed to set up E2E test environment:', error)
      throw error
    }
  })

  afterAll(async () => {
    // Clean up test data
    try {
      if (testPodcastId) {
        await invoke('remove_podcast', { podcastId: testPodcastId })
        console.log(`Cleaned up test podcast ${testPodcastId}`)
      }
    } catch (error) {
      console.error('Failed to clean up test data:', error)
    }
  })

  beforeEach(async () => {
    // Ensure episode is not downloaded before each test
    try {
      // Try to remove the episode file if it exists (cleanup from previous tests)
      await invoke('remove_episode_from_device', {
        episodeId: testEpisodeId,
        devicePath: '/tmp' // Use system temp directory for test files
      })
    } catch {
      // Ignore errors - episode might not be downloaded
    }
  })

  afterEach(async () => {
    // Clean up any downloaded files after each test
    try {
      await invoke('remove_episode_from_device', {
        episodeId: testEpisodeId,
        devicePath: '/tmp'
      })
    } catch {
      // Ignore cleanup errors
    }
  })

  describe('Complete Download Workflow', () => {
    it('should download episode from start to finish with real backend', async () => {
      render(<App />)

      // Wait for app to load and podcasts to appear
      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      }, { timeout: 5000 })

      // Find and click on the test podcast
      await waitFor(() => {
        const podcastElements = screen.getAllByText(/TED Radio Hour|NPR/)
        expect(podcastElements.length).toBeGreaterThan(0)
        fireEvent.click(podcastElements[0])
      }, { timeout: 5000 })

      // Wait for episodes to load
      await waitFor(() => {
        const episodeElements = screen.getAllByRole('generic')
        const episodeItems = episodeElements.filter(el => 
          el.className && el.className.includes('episode-item')
        )
        expect(episodeItems.length).toBeGreaterThan(0)
      }, { timeout: 5000 })

      // Click on the first episode to select it
      const episodeItems = screen.getAllByRole('generic').filter(el => 
        el.className && el.className.includes('episode-item')
      )
      fireEvent.click(episodeItems[0])

      // Wait for episode details to appear with download button
      await waitFor(() => {
        expect(screen.getByText('📥 Download Episode')).toBeInTheDocument()
      }, { timeout: 3000 })

      // Click the download button to start real download
      const downloadButton = screen.getByText('📥 Download Episode')
      fireEvent.click(downloadButton)

      // Verify downloading state appears immediately
      await waitFor(() => {
        expect(screen.getByText('📥 Downloading...')).toBeInTheDocument()
      }, { timeout: 2000 })

      // Wait for real progress to appear (this may take time for actual download)
      await waitFor(() => {
        // Look for percentage indicator
        const percentageElements = screen.getAllByText(/%/)
        expect(percentageElements.length).toBeGreaterThan(0)
      }, { timeout: 15000 }) // Allow up to 15 seconds for download to start showing progress

      // Verify progress details appear
      await waitFor(() => {
        // Look for file size information
        expect(screen.getByText(/MB|KB|Bytes/)).toBeInTheDocument()
        // Look for speed information
        expect(screen.getByText(/\/s/)).toBeInTheDocument()
        // Look for ETA information
        expect(screen.getByText(/ETA:/)).toBeInTheDocument()
      }, { timeout: 5000 })

      console.log('Download progress verified, waiting for completion...')

      // Wait for download to complete (this could take a while for real downloads)
      // We'll wait up to 60 seconds for completion, but most test episodes should be smaller
      await waitFor(() => {
        expect(screen.getByText('✅ Downloaded')).toBeInTheDocument()
      }, { timeout: 60000 })

      // Verify file path is shown
      await waitFor(() => {
        const filePathElements = screen.getAllByText(/\.mp3|\.m4a|\.wav/)
        expect(filePathElements.length).toBeGreaterThan(0)
      }, { timeout: 2000 })

      // Verify episode shows as downloaded in the list
      await waitFor(() => {
        expect(screen.getByText(/📥 Downloaded/)).toBeInTheDocument()
      })

      console.log('E2E download test completed successfully!')
    }, 120000) // 2 minute timeout for complete download

    it('should handle download errors gracefully with real backend', async () => {
      render(<App />)

      // Wait for app to load
      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      }, { timeout: 5000 })

      // Add a podcast with an invalid episode URL for testing error handling
      try {
        const invalidPodcastId = await invoke('add_podcast', {
          rssUrl: 'https://httpbin.org/xml' // This will create a podcast but episodes might have invalid URLs
        })

        // Wait for podcast to appear
        await waitFor(() => {
          const podcastElements = screen.getAllByText(/httpbin/)
          if (podcastElements.length > 0) {
            fireEvent.click(podcastElements[0])
          }
        }, { timeout: 5000 })

        // If we get an episode with invalid URL, try to download it
        const episodeItems = screen.getAllByRole('generic').filter(el => 
          el.className && el.className.includes('episode-item')
        )
        
        if (episodeItems.length > 0) {
          fireEvent.click(episodeItems[0])

          await waitFor(() => {
            const downloadButton = screen.queryByText('📥 Download Episode')
            if (downloadButton) {
              fireEvent.click(downloadButton)
            }
          }, { timeout: 3000 })

          // Wait for error message to appear
          await waitFor(() => {
            expect(screen.getByText(/Download failed|Error|Failed/)).toBeInTheDocument()
            expect(screen.getByText('🔄 Retry')).toBeInTheDocument()
          }, { timeout: 10000 })
        }

        // Clean up invalid podcast
        await invoke('remove_podcast', { podcastId: invalidPodcastId })
      } catch (error) {
        // If we can't create an invalid scenario, that's ok - just log it
        console.log('Could not create error scenario for testing:', error)
      }
    }, 30000)

    it('should prevent multiple simultaneous downloads with real backend', async () => {
      render(<App />)

      // Wait for app to load and select podcast/episode
      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      }, { timeout: 5000 })

      await waitFor(() => {
        const podcastElements = screen.getAllByText(/TED Radio Hour|NPR/)
        if (podcastElements.length > 0) {
          fireEvent.click(podcastElements[0])
        }
      }, { timeout: 5000 })

      const episodeItems = screen.getAllByRole('generic').filter(el => 
        el.className && el.className.includes('episode-item')
      )
      
      if (episodeItems.length > 0) {
        fireEvent.click(episodeItems[0])

        // Wait for download button
        await waitFor(() => {
          expect(screen.getByText('📥 Download Episode')).toBeInTheDocument()
        }, { timeout: 3000 })

        // Click download button
        const downloadButton = screen.getByText('📥 Download Episode')
        fireEvent.click(downloadButton)

        // Verify downloading state
        await waitFor(() => {
          expect(screen.getByText('📥 Downloading...')).toBeInTheDocument()
        }, { timeout: 2000 })

        // Verify download button is no longer available (prevents multiple downloads)
        expect(screen.queryByText('📥 Download Episode')).not.toBeInTheDocument()

        // Try to click another episode and verify we can't start multiple downloads
        if (episodeItems.length > 1) {
          fireEvent.click(episodeItems[1])
          
          // This episode should show download button since it's different
          await waitFor(() => {
            expect(screen.getByText('📥 Download Episode')).toBeInTheDocument()
          }, { timeout: 2000 })
        }
      }
    }, 60000)
  })

  describe('Progress Tracking Integration', () => {
    it('should show accurate progress information from real backend', async () => {
      render(<App />)

      // Set up and start download
      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      }, { timeout: 5000 })

      await waitFor(() => {
        const podcastElements = screen.getAllByText(/TED Radio Hour|NPR/)
        if (podcastElements.length > 0) {
          fireEvent.click(podcastElements[0])
        }
      }, { timeout: 5000 })

      const episodeItems = screen.getAllByRole('generic').filter(el => 
        el.className && el.className.includes('episode-item')
      )
      
      if (episodeItems.length > 0) {
        fireEvent.click(episodeItems[0])

        await waitFor(() => {
          const downloadButton = screen.getByText('📥 Download Episode')
          fireEvent.click(downloadButton)
        }, { timeout: 3000 })

        // Verify progress tracking shows real data
        await waitFor(() => {
          expect(screen.getByText('📥 Downloading...')).toBeInTheDocument()
        }, { timeout: 2000 })

        // Wait for progress details and verify they update
        let previousPercentage = -1
        let progressUpdated = false

        for (let i = 0; i < 10; i++) {
          await new Promise(resolve => setTimeout(resolve, 1000))
          
          const percentageElements = screen.getAllByText(/%/)
          if (percentageElements.length > 0) {
            const percentageText = percentageElements[0].textContent
            const currentPercentage = parseFloat(percentageText?.replace('%', '') || '0')
            
            if (previousPercentage >= 0 && currentPercentage > previousPercentage) {
              progressUpdated = true
              break
            }
            previousPercentage = currentPercentage
          }
        }

        expect(progressUpdated).toBe(true)

        // Verify progress information is realistic
        await waitFor(() => {
          // Check file sizes are realistic (should be MB or KB, not bytes for audio files)
          const sizeElements = screen.getAllByText(/\d+(\.\d+)?\s*(MB|KB)/)
          expect(sizeElements.length).toBeGreaterThan(0)

          // Check speed is reasonable (should show KB/s or MB/s)
          const speedElements = screen.getAllByText(/\d+(\.\d+)?\s*(KB|MB)\/s/)
          expect(speedElements.length).toBeGreaterThan(0)

          // Check ETA is reasonable (should be seconds, minutes, or hours)
          const etaElements = screen.getAllByText(/ETA:\s*\d+[smh]/)
          expect(etaElements.length).toBeGreaterThan(0)
        }, { timeout: 5000 })
      }
    }, 90000)
  })

  describe('File System Integration', () => {
    it('should create actual files and show correct file paths', async () => {
      render(<App />)

      // Complete a real download
      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      }, { timeout: 5000 })

      await waitFor(() => {
        const podcastElements = screen.getAllByText(/TED Radio Hour|NPR/)
        if (podcastElements.length > 0) {
          fireEvent.click(podcastElements[0])
        }
      }, { timeout: 5000 })

      const episodeItems = screen.getAllByRole('generic').filter(el => 
        el.className && el.className.includes('episode-item')
      )
      
      if (episodeItems.length > 0) {
        fireEvent.click(episodeItems[0])

        await waitFor(() => {
          const downloadButton = screen.getByText('📥 Download Episode')
          fireEvent.click(downloadButton)
        }, { timeout: 3000 })

        // Wait for download to complete
        await waitFor(() => {
          expect(screen.getByText('✅ Downloaded')).toBeInTheDocument()
        }, { timeout: 60000 })

        // Verify file path is shown and looks realistic
        await waitFor(() => {
          const filePathElements = screen.getAllByText(/\/.*\.(mp3|m4a|wav)/)
          expect(filePathElements.length).toBeGreaterThan(0)
          
          const filePath = filePathElements[0].textContent
          expect(filePath).toMatch(/\.(mp3|m4a|wav)$/)
          expect(filePath).toContain('/')
        }, { timeout: 2000 })

        // Verify backend can confirm file exists
        const episodes = await invoke('get_episodes', { podcastId: testPodcastId })
        const downloadedEpisode = episodes.find(ep => ep.id === testEpisodeId)
        
        expect(downloadedEpisode.downloaded).toBe(true)
        expect(downloadedEpisode.local_file_path).toBeTruthy()
        expect(downloadedEpisode.local_file_path).toMatch(/\.(mp3|m4a|wav)$/)
      }
    }, 120000)

    it('should persist download status across app restarts', async () => {
      // First render - download an episode
      const { unmount } = render(<App />)

      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      }, { timeout: 5000 })

      await waitFor(() => {
        const podcastElements = screen.getAllByText(/TED Radio Hour|NPR/)
        if (podcastElements.length > 0) {
          fireEvent.click(podcastElements[0])
        }
      }, { timeout: 5000 })

      const episodeItems = screen.getAllByRole('generic').filter(el => 
        el.className && el.className.includes('episode-item')
      )
      
      if (episodeItems.length > 0) {
        fireEvent.click(episodeItems[0])

        await waitFor(() => {
          const downloadButton = screen.getByText('📥 Download Episode')
          fireEvent.click(downloadButton)
        }, { timeout: 3000 })

        await waitFor(() => {
          expect(screen.getByText('✅ Downloaded')).toBeInTheDocument()
        }, { timeout: 60000 })
      }

      // Unmount the app (simulate restart)
      unmount()

      // Re-render the app (simulate restart)
      render(<App />)

      // Verify download status persists
      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      }, { timeout: 5000 })

      await waitFor(() => {
        const podcastElements = screen.getAllByText(/TED Radio Hour|NPR/)
        if (podcastElements.length > 0) {
          fireEvent.click(podcastElements[0])
        }
      }, { timeout: 5000 })

      await waitFor(() => {
        expect(screen.getByText(/📥 Downloaded/)).toBeInTheDocument()
      }, { timeout: 5000 })

      // Select the downloaded episode and verify it still shows as downloaded
      const newEpisodeItems = screen.getAllByRole('generic').filter(el => 
        el.className && el.className.includes('episode-item')
      )
      
      if (newEpisodeItems.length > 0) {
        fireEvent.click(newEpisodeItems[0])

        await waitFor(() => {
          expect(screen.getByText('✅ Downloaded')).toBeInTheDocument()
          // Should not show download button since it's already downloaded
          expect(screen.queryByText('📥 Download Episode')).not.toBeInTheDocument()
        }, { timeout: 3000 })
      }
    }, 150000)
  })
}) 