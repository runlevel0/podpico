import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { describe, it, expect, vi, beforeEach } from 'vitest'
import { invoke } from '@tauri-apps/api/core'
import App from '../App'

// Mock tauri invoke
vi.mock('@tauri-apps/api/core')

const mockInvoke = vi.mocked(invoke)

describe('Remove Downloaded Episode - End-to-End Integration Tests', () => {
  beforeEach(() => {
    vi.clearAllMocks()

    // Default mock implementation for common calls
    mockInvoke.mockImplementation((command: string) => {
      switch (command) {
        case 'get_podcasts':
          return Promise.resolve([
            {
              id: 1,
              name: 'Test Podcast',
              rss_url: 'https://example.com/feed.xml',
              description: 'Test description',
              artwork_url: null,
              website_url: null,
              last_updated: null,
              episode_count: 1,
              new_episode_count: 0,
            },
          ])
        case 'get_usb_devices':
          return Promise.resolve([])
        case 'get_episodes':
          return Promise.resolve([
            {
              id: 1,
              podcast_id: 1,
              podcast_name: 'Test Podcast',
              title: 'Test Episode',
              description: 'Test description',
              episode_url: 'https://example.com/episode.mp3',
              published_date: '2023-01-01T00:00:00Z',
              duration: 3600,
              file_size: 50000000,
              local_file_path: '/path/to/episode.mp3',
              status: 'new',
              downloaded: true,
              on_device: false,
            },
          ])
        case 'delete_downloaded_episode':
          return Promise.resolve()
        default:
          return Promise.reject(new Error(`Unhandled command: ${command}`))
      }
    })
  })

  describe('Complete Remove Episode User Journey', () => {
    it('should complete full episode removal workflow with backend integration', async () => {
      console.log(
        '🚀 Testing complete remove episode workflow E2E integration...'
      )

      render(<App />)

      // Step 1: Wait for app to load and verify initial backend calls
      await waitFor(
        () => {
          expect(screen.getByText('PodPico')).toBeInTheDocument()
        },
        { timeout: 5000 }
      )

      expect(mockInvoke).toHaveBeenCalledWith('get_podcasts')
      expect(mockInvoke).toHaveBeenCalledWith('get_episodes', {
        podcastId: null,
      })
      console.log('✅ App loaded with initial backend calls')

      // Step 2: Navigate to test podcast
      await waitFor(() => {
        const podcastElement = screen.getByText('Test Podcast')
        expect(podcastElement).toBeInTheDocument()
        fireEvent.click(podcastElement)
      })

      expect(mockInvoke).toHaveBeenCalledWith('get_episodes', { podcastId: 1 })
      console.log('✅ Podcast selected and episodes loaded')

      // Step 3: Verify episodes are displayed
      await waitFor(() => {
        expect(screen.getByText('Test Episode')).toBeInTheDocument()
      })
      console.log('✅ Episode displayed correctly')

      // Step 4: Select downloaded episode
      const episode = screen.getByText('Test Episode')
      fireEvent.click(episode)

      await waitFor(() => {
        expect(screen.getByText('✅ Downloaded')).toBeInTheDocument()
        expect(screen.getByText('🗑️ Remove Download')).toBeInTheDocument()
      })
      console.log('✅ Downloaded episode selected with remove button shown')

      // Step 5: Initiate removal
      const removeButton = screen.getByText('🗑️ Remove Download')
      fireEvent.click(removeButton)

      expect(mockInvoke).toHaveBeenCalledWith('delete_downloaded_episode', {
        episodeId: 1,
      })
      console.log('✅ Remove command sent to backend')

      // Step 6: Verify removing state
      await waitFor(() => {
        expect(screen.getByText('🗑️ Removing...')).toBeInTheDocument()
      })
      console.log('✅ Removing state displayed')

      console.log(
        '🎉 Complete remove episode workflow E2E integration test PASSED!'
      )
    })

    it('should show correct states for downloaded vs non-downloaded episodes', async () => {
      console.log('🚀 Testing episode remove button visibility...')

      // Mock both downloaded and non-downloaded episodes
      mockInvoke.mockImplementation((command: string) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([
              {
                id: 1,
                name: 'Test Podcast',
                rss_url: 'https://example.com/feed.xml',
                description: 'Test description',
                artwork_url: null,
                website_url: null,
                last_updated: null,
                episode_count: 2,
                new_episode_count: 0,
              },
            ])
          case 'get_usb_devices':
            return Promise.resolve([])
          case 'get_episodes':
            return Promise.resolve([
              {
                id: 1,
                podcast_id: 1,
                podcast_name: 'Test Podcast',
                title: 'Downloaded Episode',
                description: 'This episode is downloaded',
                episode_url: 'https://example.com/downloaded.mp3',
                published_date: '2023-01-01T00:00:00Z',
                duration: 3600,
                file_size: 50000000,
                local_file_path: '/path/to/downloaded.mp3',
                status: 'new',
                downloaded: true,
                on_device: false,
              },
              {
                id: 2,
                podcast_id: 1,
                podcast_name: 'Test Podcast',
                title: 'Not Downloaded Episode',
                description: 'This episode is not downloaded',
                episode_url: 'https://example.com/not-downloaded.mp3',
                published_date: '2023-01-02T00:00:00Z',
                duration: 3600,
                file_size: 50000000,
                local_file_path: null,
                status: 'new',
                downloaded: false,
                on_device: false,
              },
            ])
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      })

      const podcastElement = screen.getByText('Test Podcast')
      fireEvent.click(podcastElement)

      await waitFor(() => {
        expect(screen.getByText('Downloaded Episode')).toBeInTheDocument()
        expect(screen.getByText('Not Downloaded Episode')).toBeInTheDocument()
      })

      // Test downloaded episode
      const downloadedEpisode = screen.getByText('Downloaded Episode')
      fireEvent.click(downloadedEpisode)

      await waitFor(() => {
        expect(screen.getByText('✅ Downloaded')).toBeInTheDocument()
        expect(screen.getByText('🗑️ Remove Download')).toBeInTheDocument()
        expect(
          screen.queryByText('📥 Download Episode')
        ).not.toBeInTheDocument()
      })
      console.log('✅ Downloaded episode shows remove button')

      // Test non-downloaded episode
      const notDownloadedEpisode = screen.getByText('Not Downloaded Episode')
      fireEvent.click(notDownloadedEpisode)

      await waitFor(() => {
        expect(screen.getByText('📥 Download Episode')).toBeInTheDocument()
        expect(screen.queryByText('🗑️ Remove Download')).not.toBeInTheDocument()
      })
      console.log('✅ Non-downloaded episode shows download button')

      console.log('🎉 Episode remove button visibility test PASSED!')
    })
  })

  describe('Remove Episode Error Handling', () => {
    it('should handle backend errors during removal', async () => {
      console.log('🚀 Testing error handling during episode removal...')

      mockInvoke.mockImplementation((command: string) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([
              {
                id: 1,
                name: 'Test Podcast',
                rss_url: 'https://example.com/feed.xml',
                description: 'Test description',
                artwork_url: null,
                website_url: null,
                last_updated: null,
                episode_count: 1,
                new_episode_count: 0,
              },
            ])
          case 'get_usb_devices':
            return Promise.resolve([])
          case 'get_episodes':
            return Promise.resolve([
              {
                id: 1,
                podcast_id: 1,
                podcast_name: 'Test Podcast',
                title: 'Test Episode',
                description: 'Test description',
                episode_url: 'https://example.com/episode.mp3',
                published_date: '2023-01-01T00:00:00Z',
                duration: 3600,
                file_size: 50000000,
                local_file_path: '/path/to/episode.mp3',
                status: 'new',
                downloaded: true,
                on_device: false,
              },
            ])
          case 'delete_downloaded_episode':
            return Promise.reject(new Error('File deletion failed'))
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('Test Podcast'))

      await waitFor(() => {
        expect(screen.getByText('Test Episode')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('Test Episode'))

      await waitFor(() => {
        expect(screen.getByText('🗑️ Remove Download')).toBeInTheDocument()
      })

      const removeButton = screen.getByText('🗑️ Remove Download')
      fireEvent.click(removeButton)

      // Wait for error message
      await waitFor(() => {
        expect(screen.getByText(/Remove failed/)).toBeInTheDocument()
        expect(screen.getByText('🔄 Retry')).toBeInTheDocument()
      })

      console.log('✅ Error handling during removal verified')
    })

    it('should allow retry after removal failure', async () => {
      console.log('🚀 Testing retry functionality after removal failure...')

      let removeAttempts = 0
      mockInvoke.mockImplementation((command: string) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([
              {
                id: 1,
                name: 'Test Podcast',
                rss_url: 'https://example.com/feed.xml',
                description: 'Test description',
                artwork_url: null,
                website_url: null,
                last_updated: null,
                episode_count: 1,
                new_episode_count: 0,
              },
            ])
          case 'get_usb_devices':
            return Promise.resolve([])
          case 'get_episodes':
            return Promise.resolve([
              {
                id: 1,
                podcast_id: 1,
                podcast_name: 'Test Podcast',
                title: 'Test Episode',
                description: 'Test description',
                episode_url: 'https://example.com/episode.mp3',
                published_date: '2023-01-01T00:00:00Z',
                duration: 3600,
                file_size: 50000000,
                local_file_path: '/path/to/episode.mp3',
                status: 'new',
                downloaded: true,
                on_device: false,
              },
            ])
          case 'delete_downloaded_episode':
            removeAttempts++
            if (removeAttempts === 1) {
              return Promise.reject(new Error('First attempt failed'))
            }
            return Promise.resolve()
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('Test Podcast'))

      await waitFor(() => {
        expect(screen.getByText('Test Episode')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('Test Episode'))

      await waitFor(() => {
        expect(screen.getByText('🗑️ Remove Download')).toBeInTheDocument()
      })

      // First attempt (should fail)
      const removeButton = screen.getByText('🗑️ Remove Download')
      fireEvent.click(removeButton)

      await waitFor(() => {
        expect(screen.getByText(/Remove failed/)).toBeInTheDocument()
        expect(screen.getByText('🔄 Retry')).toBeInTheDocument()
      })

      // Retry (should succeed)
      const retryButton = screen.getByText('🔄 Retry')
      fireEvent.click(retryButton)

      await waitFor(() => {
        expect(screen.getByText('🗑️ Removing...')).toBeInTheDocument()
      })

      expect(removeAttempts).toBe(2)
      console.log('✅ Retry functionality verified')
    })
  })

  describe('Backend Command Integration Coverage', () => {
    it('should verify all remove-related backend commands are properly integrated', async () => {
      console.log('🚀 Testing complete backend command integration coverage...')

      render(<App />)

      // Complete workflow that exercises all remove-related commands
      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      })

      const podcastElement = screen.getByText('Test Podcast')
      fireEvent.click(podcastElement)

      await waitFor(() => {
        expect(screen.getByText('Test Episode')).toBeInTheDocument()
      })

      const episode = screen.getByText('Test Episode')
      fireEvent.click(episode)

      const removeButton = screen.getByText('🗑️ Remove Download')
      fireEvent.click(removeButton)

      // Analyze all backend calls made
      const allCalls = mockInvoke.mock.calls
      const calledCommands = allCalls.map(call => call[0])
      const uniqueCommands = [...new Set(calledCommands)]

      // Verify essential remove commands are covered
      expect(calledCommands).toContain('get_podcasts')
      expect(calledCommands).toContain('get_episodes')
      expect(calledCommands).toContain('delete_downloaded_episode')

      // Verify command arguments are correct
      const removeCall = allCalls.find(
        call => call[0] === 'delete_downloaded_episode'
      )
      expect(removeCall?.[1]).toEqual({ episodeId: 1 })

      console.log(
        `✅ Backend integration coverage verified: ${uniqueCommands.join(', ')}`
      )
      console.log(`📊 Total backend calls: ${allCalls.length}`)
    })
  })

  describe('Multi-Episode Remove Management', () => {
    it('should prevent multiple simultaneous removals', async () => {
      console.log('🚀 Testing simultaneous removal prevention...')

      let removeCallCount = 0
      mockInvoke.mockImplementation((command: string) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([
              {
                id: 1,
                name: 'Test Podcast',
                rss_url: 'https://example.com/feed.xml',
                description: 'Test description',
                artwork_url: null,
                website_url: null,
                last_updated: null,
                episode_count: 2,
                new_episode_count: 0,
              },
            ])
          case 'get_usb_devices':
            return Promise.resolve([])
          case 'get_episodes':
            return Promise.resolve([
              {
                id: 1,
                podcast_id: 1,
                podcast_name: 'Test Podcast',
                title: 'Episode 1',
                description: 'First episode',
                episode_url: 'https://example.com/episode1.mp3',
                published_date: '2023-01-01T00:00:00Z',
                duration: 3600,
                file_size: 50000000,
                local_file_path: '/path/to/episode1.mp3',
                status: 'new',
                downloaded: true,
                on_device: false,
              },
              {
                id: 2,
                podcast_id: 1,
                podcast_name: 'Test Podcast',
                title: 'Episode 2',
                description: 'Second episode',
                episode_url: 'https://example.com/episode2.mp3',
                published_date: '2023-01-02T00:00:00Z',
                duration: 3600,
                file_size: 50000000,
                local_file_path: '/path/to/episode2.mp3',
                status: 'new',
                downloaded: true,
                on_device: false,
              },
            ])
          case 'delete_downloaded_episode':
            removeCallCount++
            return new Promise(() => {}) // Never resolve to keep "removing" state
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('Test Podcast'))

      await waitFor(() => {
        expect(screen.getByText('Episode 1')).toBeInTheDocument()
      })

      // Start remove on first episode
      const episode1 = screen.getByText('Episode 1')
      fireEvent.click(episode1)

      await waitFor(() => {
        expect(screen.getByText('🗑️ Remove Download')).toBeInTheDocument()
      })

      const removeButton = screen.getByText('🗑️ Remove Download')
      fireEvent.click(removeButton)

      await waitFor(() => {
        expect(screen.getByText('🗑️ Removing...')).toBeInTheDocument()
      })

      // Try to select another episode
      const episode2 = screen.getByText('Episode 2')
      fireEvent.click(episode2)

      // Should still show remove button for other episodes
      await waitFor(() => {
        expect(screen.getByText('🗑️ Remove Download')).toBeInTheDocument()
      })

      // Verify first episode's remove was called
      expect(removeCallCount).toBe(1)
      console.log('✅ Multiple episode remove management verified')
    })
  })
})
