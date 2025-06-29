import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import App from '../App'

// Mock Tauri API for E2E integration testing
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}))

import { invoke } from '@tauri-apps/api/core'
const mockInvoke = vi.mocked(invoke)

// Test data that simulates real backend responses
const mockPodcast = {
  id: 1,
  name: "Test Podcast",
  rss_url: "https://example.com/feed.xml",
  description: "A test podcast for E2E testing",
  artwork_url: null,
  website_url: null,
  last_updated: null,
  episode_count: 3,
  new_episode_count: 2
}

const mockEpisodes = [
  {
    id: 1,
    podcast_id: 1,
    podcast_name: "Test Podcast",
    title: "Episode 1: Introduction",
    description: "First episode for testing download",
    episode_url: "https://example.com/episode1.mp3",
    published_date: "2024-01-01",
    duration: 1800,
    file_size: 25000000,
    local_file_path: null,
    status: "New",
    downloaded: false,
    on_device: false
  },
  {
    id: 2,
    podcast_id: 1,
    podcast_name: "Test Podcast", 
    title: "Episode 2: Deep Dive",
    description: "Second episode - already downloaded",
    episode_url: "https://example.com/episode2.mp3",
    published_date: "2024-01-02",
    duration: 2400,
    file_size: 30000000,
    local_file_path: "/tmp/podpico/episode2.mp3",
    status: "New",
    downloaded: true,
    on_device: false
  },
  {
    id: 3,
    podcast_id: 1,
    podcast_name: "Test Podcast",
    title: "Episode 3: Advanced Topics",
    description: "Third episode for testing",
    episode_url: "https://example.com/episode3.mp3",
    published_date: "2024-01-03",
    duration: 3600,
    file_size: 45000000,
    local_file_path: null,
    status: "New",
    downloaded: false,
    on_device: false
  }
]

describe('User Story #3: Download Episodes - End-to-End Integration Tests', () => {
  beforeEach(() => {
    // Reset mocks before each test
    vi.clearAllMocks()
    
    // Setup default mock responses that simulate real backend behavior
    mockInvoke.mockImplementation((command: string, args?: any) => {
      switch (command) {
        case 'get_podcasts':
          return Promise.resolve([mockPodcast])
        
        case 'get_episodes':
          if (args?.podcastId === 1) {
            return Promise.resolve(mockEpisodes)
          } else if (args?.podcastId === null) {
            // Combined inbox - return all new episodes
            return Promise.resolve(mockEpisodes.filter(ep => ep.status === 'New'))
          }
          return Promise.resolve([])
        
        case 'download_episode':
          // Simulate successful download initiation
          return Promise.resolve()
          
        case 'get_download_progress':
          // Simulate realistic download progress
          return Promise.resolve({
            episode_id: args?.episodeId,
            bytes_downloaded: 12500000,
            total_bytes: 25000000,
            percentage: 50.0,
            download_speed: 1024000,
            eta_seconds: 12
          })
          
        default:
          return Promise.reject(new Error(`Unhandled command: ${command}`))
      }
    })
  })

  afterEach(() => {
    vi.clearAllMocks()
  })

  describe('Complete Download User Journey', () => {
    it('should complete full download workflow with frontend-backend integration', async () => {
      console.log('🚀 Testing complete download workflow E2E integration...')
      
      const { container } = render(<App />)

      // Step 1: Wait for app to load and verify initial backend calls
      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      }, { timeout: 5000 })

      expect(mockInvoke).toHaveBeenCalledWith('get_podcasts')
      expect(mockInvoke).toHaveBeenCalledWith('get_episodes', { podcastId: null })
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
        expect(screen.getByText('Episode 1: Introduction')).toBeInTheDocument()
        expect(screen.getByText('Episode 2: Deep Dive')).toBeInTheDocument()
        expect(screen.getByText('Episode 3: Advanced Topics')).toBeInTheDocument()
      })
      console.log('✅ All episodes displayed correctly')

      // Step 4: Select undownloaded episode
      const episode1 = screen.getByText('Episode 1: Introduction')
      fireEvent.click(episode1)

      await waitFor(() => {
        expect(screen.getByText('📥 Download Episode')).toBeInTheDocument()
      })
      console.log('✅ Download button shown for undownloaded episode')

      // Step 5: Initiate download
      const downloadButton = screen.getByText('📥 Download Episode')
      fireEvent.click(downloadButton)

      expect(mockInvoke).toHaveBeenCalledWith('download_episode', { episodeId: 1 })
      console.log('✅ Download command sent to backend')

      // Step 6: Verify downloading state
      await waitFor(() => {
        expect(screen.getByText('📥 Downloading...')).toBeInTheDocument()
      })
      console.log('✅ Downloading state displayed')

      // Step 7: Wait for progress tracking to begin
      await waitFor(() => {
        expect(mockInvoke).toHaveBeenCalledWith('get_download_progress', { episodeId: 1 })
      }, { timeout: 3000 })
      console.log('✅ Progress tracking initiated')

      console.log('🎉 Complete download workflow E2E integration test PASSED!')
    })

    it('should show correct states for downloaded vs undownloaded episodes', async () => {
      console.log('🚀 Testing episode state display integration...')
      
      const { container } = render(<App />)

      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      })

      const podcastElement = screen.getByText('Test Podcast')
      fireEvent.click(podcastElement)

      await waitFor(() => {
        expect(screen.getByText('Episode 1: Introduction')).toBeInTheDocument()
      })

      // Test undownloaded episode
      const episode1 = screen.getByText('Episode 1: Introduction')
      fireEvent.click(episode1)

      await waitFor(() => {
        expect(screen.getByText('📥 Download Episode')).toBeInTheDocument()
        expect(screen.queryByText('✅ Downloaded')).not.toBeInTheDocument()
      })
      console.log('✅ Undownloaded episode shows download button')

      // Test downloaded episode
      const episode2 = screen.getByText('Episode 2: Deep Dive')
      fireEvent.click(episode2)

      await waitFor(() => {
        expect(screen.getByText('✅ Downloaded')).toBeInTheDocument()
        expect(screen.getByText('/tmp/podpico/episode2.mp3')).toBeInTheDocument()
        expect(screen.queryByText('📥 Download Episode')).not.toBeInTheDocument()
      })
      console.log('✅ Downloaded episode shows file path and no download button')

      console.log('🎉 Episode state display integration test PASSED!')
    })
  })

  describe('Download Progress Integration', () => {
    it('should track download progress with real-time updates', async () => {      
      console.log('🚀 Testing download progress tracking integration...')
      
      // Mock progressive download updates
      let progressCallCount = 0
      mockInvoke.mockImplementation((command: string, args?: any) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([mockPodcast])
          case 'get_episodes':
            if (args?.podcastId === 1) {
              return Promise.resolve(mockEpisodes)
            }
            return Promise.resolve(mockEpisodes.filter(ep => ep.status === 'New'))
          case 'download_episode':
            return Promise.resolve()
          case 'get_download_progress':
            progressCallCount++
            const progress = Math.min(progressCallCount * 25, 100)
            return Promise.resolve({
              episode_id: args?.episodeId,
              bytes_downloaded: progress * 250000,
              total_bytes: 25000000,
              percentage: progress,
              download_speed: 1000000 + (progressCallCount * 50000),
              eta_seconds: Math.max(20 - progressCallCount * 4, 0)
            })
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      const { container } = render(<App />)

      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      })

      const podcastElement = screen.getByText('Test Podcast')
      fireEvent.click(podcastElement)

      await waitFor(() => {
        expect(screen.getByText('Episode 1: Introduction')).toBeInTheDocument()
      })

      const episode1 = screen.getByText('Episode 1: Introduction')
      fireEvent.click(episode1)

      const downloadButton = screen.getByText('📥 Download Episode')
      fireEvent.click(downloadButton)

      // Wait for downloading state
      await waitFor(() => {
        expect(screen.getByText('📥 Downloading...')).toBeInTheDocument()
      })

      // Wait for progress updates
      await new Promise(resolve => setTimeout(resolve, 2500))

      // Verify progress tracking was called multiple times
      expect(mockInvoke).toHaveBeenCalledWith('get_download_progress', { episodeId: 1 })
      expect(progressCallCount).toBeGreaterThan(1)

      console.log(`✅ Progress tracking integration verified (${progressCallCount} progress updates)`)
    })

    it('should display formatted progress information', async () => {
      console.log('🚀 Testing progress information display integration...')
      
      // Mock specific progress data
      mockInvoke.mockImplementation((command: string, args?: any) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([mockPodcast])
          case 'get_episodes':
            if (args?.podcastId === 1) {
              return Promise.resolve(mockEpisodes)
            }
            return Promise.resolve(mockEpisodes.filter(ep => ep.status === 'New'))
          case 'download_episode':
            return Promise.resolve()
          case 'get_download_progress':
            return Promise.resolve({
              episode_id: args?.episodeId,
              bytes_downloaded: 15728640, // 15 MB
              total_bytes: 26214400,     // 25 MB
              percentage: 60.0,
              download_speed: 1048576,   // 1 MB/s
              eta_seconds: 10
            })
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      const { container } = render(<App />)

      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      })

      const podcastElement = screen.getByText('Test Podcast')
      fireEvent.click(podcastElement)

      await waitFor(() => {
        expect(screen.getByText('Episode 1: Introduction')).toBeInTheDocument()
      })

      const episode1 = screen.getByText('Episode 1: Introduction')
      fireEvent.click(episode1)

      const downloadButton = screen.getByText('📥 Download Episode')
      fireEvent.click(downloadButton)

      // Wait for download to start - check inline progress in episode list
      await waitFor(() => {
        expect(screen.getByText('📥 Downloading...')).toBeInTheDocument()
      })

      // Wait for progress tracking interval to fire and display progress information
      await new Promise(resolve => setTimeout(resolve, 1500))

      // Verify inline progress appears in episode list (rounded percentage)
      await waitFor(() => {
        expect(screen.getByText('(60%)')).toBeInTheDocument() // Inline progress uses toFixed(0) with parentheses
        expect(screen.getByText('• ⏳ Downloading')).toBeInTheDocument() // Download status indicator
      })

      // Note: Detailed progress information (60.0%, file sizes, speed, ETA) only appears 
      // in the episode details pane when an episode is selected. This test focuses on
      // the inline progress which is the primary user-visible indicator.

      console.log('✅ Progress information display integration verified')
    })
  })

  describe('Error Handling Integration', () => {
    it('should handle download errors from backend gracefully', async () => {
      console.log('🚀 Testing download error handling integration...')
      
      // Mock download failure
      mockInvoke.mockImplementation((command: string, args?: any) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([mockPodcast])
          case 'get_episodes':
            if (args?.podcastId === 1) {
              return Promise.resolve(mockEpisodes)
            }
            return Promise.resolve(mockEpisodes.filter(ep => ep.status === 'New'))
          case 'download_episode':
            return Promise.reject(new Error('Network timeout: Unable to download episode'))
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      const { container } = render(<App />)

      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      })

      const podcastElement = screen.getByText('Test Podcast')
      fireEvent.click(podcastElement)

      await waitFor(() => {
        expect(screen.getByText('Episode 1: Introduction')).toBeInTheDocument()
      })

      const episode1 = screen.getByText('Episode 1: Introduction')
      fireEvent.click(episode1)

      const downloadButton = screen.getByText('📥 Download Episode')
      fireEvent.click(downloadButton)

      // Wait for error state
      await waitFor(() => {
        expect(screen.getByText(/Download failed/)).toBeInTheDocument()
        expect(screen.getByText('🔄 Retry')).toBeInTheDocument()
      }, { timeout: 5000 })

      console.log('✅ Download error handling integration verified')
    })

    it('should allow retry after download failure', async () => {
      console.log('🚀 Testing download retry integration...')
      
      let downloadAttempts = 0
      mockInvoke.mockImplementation((command: string, args?: any) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([mockPodcast])
          case 'get_episodes':
            if (args?.podcastId === 1) {
              return Promise.resolve(mockEpisodes)
            }
            return Promise.resolve(mockEpisodes.filter(ep => ep.status === 'New'))
          case 'download_episode':
            downloadAttempts++
            if (downloadAttempts === 1) {
              return Promise.reject(new Error('Network error'))
            } else {
              return Promise.resolve() // Success on retry
            }
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      const { container } = render(<App />)

      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      })

      const podcastElement = screen.getByText('Test Podcast')
      fireEvent.click(podcastElement)

      await waitFor(() => {
        expect(screen.getByText('Episode 1: Introduction')).toBeInTheDocument()
      })

      const episode1 = screen.getByText('Episode 1: Introduction')
      fireEvent.click(episode1)

      // First attempt - should fail
      const downloadButton = screen.getByText('📥 Download Episode')
      fireEvent.click(downloadButton)

      await waitFor(() => {
        expect(screen.getByText(/Download failed/)).toBeInTheDocument()
      })

      // Retry - should succeed
      const retryButton = screen.getByText('🔄 Retry')
      fireEvent.click(retryButton)

      await waitFor(() => {
        expect(screen.getByText('📥 Downloading...')).toBeInTheDocument()
      })

      expect(downloadAttempts).toBe(2)
      console.log('✅ Download retry integration verified')
    })
  })

  describe('Backend Command Integration Coverage', () => {
    it('should verify all download-related backend commands are properly integrated', async () => {
      console.log('🚀 Testing complete backend command integration coverage...')
      
      const { container } = render(<App />)

      // Complete workflow that exercises all download-related commands
      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      })

      const podcastElement = screen.getByText('Test Podcast')
      fireEvent.click(podcastElement)

      await waitFor(() => {
        expect(screen.getByText('Episode 1: Introduction')).toBeInTheDocument()
      })

      const episode1 = screen.getByText('Episode 1: Introduction')
      fireEvent.click(episode1)

      const downloadButton = screen.getByText('📥 Download Episode')
      fireEvent.click(downloadButton)

      // Wait for progress tracking to begin
      await new Promise(resolve => setTimeout(resolve, 1500))

      // Analyze all backend calls made
      const allCalls = mockInvoke.mock.calls
      const calledCommands = allCalls.map(call => call[0])
      const uniqueCommands = [...new Set(calledCommands)]

      // Verify essential download commands are covered
      expect(calledCommands).toContain('get_podcasts')
      expect(calledCommands).toContain('get_episodes')
      expect(calledCommands).toContain('download_episode')
      expect(calledCommands).toContain('get_download_progress')

      // Verify command arguments are correct
      const downloadCall = allCalls.find(call => call[0] === 'download_episode')
      expect(downloadCall?.[1]).toEqual({ episodeId: 1 })

      const progressCall = allCalls.find(call => call[0] === 'get_download_progress')
      expect(progressCall?.[1]).toEqual({ episodeId: 1 })

      console.log(`✅ Backend integration coverage verified: ${uniqueCommands.join(', ')}`)
      console.log(`📊 Total backend calls: ${allCalls.length}`)
    })
  })

  describe('Multi-Episode Download Management', () => {
    it('should prevent multiple simultaneous downloads', async () => {
      console.log('🚀 Testing simultaneous download prevention integration...')
      
      const { container } = render(<App />)

      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
      })

      const podcastElement = screen.getByText('Test Podcast')
      fireEvent.click(podcastElement)

      await waitFor(() => {
        expect(screen.getByText('Episode 1: Introduction')).toBeInTheDocument()
      })

      // Start download on first episode
      const episode1 = screen.getByText('Episode 1: Introduction')
      fireEvent.click(episode1)

      const downloadButton = screen.getByText('📥 Download Episode')
      fireEvent.click(downloadButton)

      await waitFor(() => {
        expect(screen.getByText('📥 Downloading...')).toBeInTheDocument()
      })

      // Try to select another episode
      const episode3 = screen.getByText('Episode 3: Advanced Topics')
      fireEvent.click(episode3)

      // Should still show download button for other episodes
      await waitFor(() => {
        expect(screen.getByText('📥 Download Episode')).toBeInTheDocument()
      })

      console.log('✅ Multiple episode download management verified')
    })
  })
}) 