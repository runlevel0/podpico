import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import App from '../App'

// Mock Tauri API for E2E integration testing
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

import { invoke } from '@tauri-apps/api/core'
const mockInvoke = vi.mocked(invoke)

// Test data that simulates real backend USB device responses
const mockUsbDevices = [
  {
    id: 'usb-device-1',
    name: 'SanDisk Ultra USB 3.0',
    path: '/media/patrick/SANDISK',
    total_space: 32000000000, // 32GB
    available_space: 16000000000, // 16GB available (50% full)
    is_connected: true,
  },
  {
    id: 'usb-device-2',
    name: 'Kingston DataTraveler',
    path: '/media/patrick/KINGSTON',
    total_space: 8000000000, // 8GB
    available_space: 2000000000, // 2GB available (75% full)
    is_connected: true,
  },
]

const mockPodcast = {
  id: 1,
  name: 'Test Podcast',
  rss_url: 'https://example.com/feed.xml',
  description: 'A test podcast for USB E2E testing',
  artwork_url: null,
  website_url: null,
  last_updated: null,
  episode_count: 2,
  new_episode_count: 2,
}

const mockEpisodes = [
  {
    id: 1,
    podcast_id: 1,
    podcast_name: 'Test Podcast',
    title: 'Episode 1: Introduction',
    description: 'First episode for USB testing',
    episode_url: 'https://example.com/episode1.mp3',
    published_date: '2024-01-01',
    duration: 1800,
    file_size: 25000000, // 25MB
    local_file_path: '/tmp/podpico/episode1.mp3',
    status: 'new',
    downloaded: true,
    on_device: false,
  },
  {
    id: 2,
    podcast_id: 1,
    podcast_name: 'Test Podcast',
    title: 'Episode 2: Deep Dive',
    description: 'Second episode for USB testing',
    episode_url: 'https://example.com/episode2.mp3',
    published_date: '2024-01-02',
    duration: 2400,
    file_size: 30000000, // 30MB
    local_file_path: '/tmp/podpico/episode2.mp3',
    status: 'new',
    downloaded: true,
    on_device: false,
  },
]

describe('User Story #8: USB Device Management - End-to-End Integration Tests', () => {
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
            return Promise.resolve(
              mockEpisodes.filter(ep => ep.status === 'new')
            )
          }
          return Promise.resolve([])

        case 'get_usb_devices':
          // Simulate USB device detection
          return Promise.resolve(mockUsbDevices)

        default:
          return Promise.reject(new Error(`Unhandled command: ${command}`))
      }
    })
  })

  afterEach(() => {
    vi.clearAllMocks()
  })

  describe('Complete USB Device Detection User Journey', () => {
    it('should complete full USB device detection workflow with frontend-backend integration', async () => {
      console.log(
        '🚀 Testing complete USB device detection workflow E2E integration...'
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
      expect(mockInvoke).toHaveBeenCalledWith('get_usb_devices')
      console.log(
        '✅ App loaded with initial backend calls including USB detection'
      )

      // Step 2: Verify USB Device section is displayed
      await waitFor(() => {
        expect(screen.getByText('USB Device')).toBeInTheDocument()
      })
      console.log('✅ USB Device section displayed in sidebar')

      // Step 3: Verify USB devices are detected and displayed
      await waitFor(() => {
        expect(screen.getByText('SanDisk Ultra USB 3.0')).toBeInTheDocument()
        expect(screen.getByText('Kingston DataTraveler')).toBeInTheDocument()
      })
      console.log('✅ USB devices detected and displayed correctly')

      // Step 4: Verify storage information is displayed correctly
      await waitFor(() => {
        // Check for storage display - SanDisk has ~14.9GB available of ~29.8GB total (formatted)
        expect(screen.getByText(/14\.9 GB/)).toBeInTheDocument() // Available space (formatted)
        expect(screen.getByText(/29\.8 GB/)).toBeInTheDocument() // Total space (formatted)
        const availableTexts = screen.getAllByText(/available/)
        expect(availableTexts.length).toBeGreaterThan(0) // Multiple devices show "available"
      })
      console.log('✅ Storage capacity information displayed correctly')

      // Step 5: Verify progress bars are shown with correct accessibility
      const progressBars = screen.getAllByRole('progressbar')
      expect(progressBars.length).toBeGreaterThan(0)

      // SanDisk should show 50% usage (16GB used out of 32GB total)
      const sandiskProgressBar = progressBars.find(bar =>
        bar.getAttribute('aria-label')?.includes('50%')
      )
      expect(sandiskProgressBar).toBeDefined()
      console.log(
        '✅ Progress bars displayed with correct percentages and accessibility attributes'
      )

      console.log(
        '🎉 Complete USB device detection E2E integration test PASSED!'
      )
    })

    it('should handle USB device detection performance requirements', async () => {
      console.log(
        '🚀 Testing USB device detection performance E2E integration...'
      )

      // Add realistic delay to simulate detection time
      mockInvoke.mockImplementation((command: string) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([mockPodcast])
          case 'get_episodes':
            return Promise.resolve(mockEpisodes)
          case 'get_usb_devices':
            // Simulate 2-second detection time (should be under 5 seconds per acceptance criteria)
            return new Promise(resolve => {
              setTimeout(() => resolve(mockUsbDevices), 2000)
            })
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      const startTime = Date.now()
      render(<App />)

      // Wait for USB devices to be detected and displayed
      await waitFor(
        () => {
          expect(screen.getByText('SanDisk Ultra USB 3.0')).toBeInTheDocument()
        },
        { timeout: 6000 }
      ) // Allow up to 6 seconds but should complete in ~2

      const detectionTime = Date.now() - startTime
      expect(detectionTime).toBeLessThan(5000) // User Story #8 acceptance criteria
      console.log(
        `✅ USB detection completed in ${detectionTime}ms (under 5 second requirement)`
      )

      console.log('🎉 USB device detection performance E2E test PASSED!')
    })

    it('should handle no USB devices connected scenario', async () => {
      console.log('🚀 Testing no USB devices connected E2E integration...')

      // Mock empty USB devices response
      mockInvoke.mockImplementation((command: string) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([mockPodcast])
          case 'get_episodes':
            return Promise.resolve(mockEpisodes)
          case 'get_usb_devices':
            return Promise.resolve([]) // No USB devices
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('USB Device')).toBeInTheDocument()
      })

      // Verify "No device connected" message is shown
      await waitFor(() => {
        expect(screen.getByText('📱 No device connected')).toBeInTheDocument()
      })
      console.log('✅ No device connected message displayed correctly')

      console.log('🎉 No USB devices E2E test PASSED!')
    })

    it('should handle USB device detection errors gracefully', async () => {
      console.log(
        '🚀 Testing USB device detection error handling E2E integration...'
      )

      // Mock USB detection failure
      mockInvoke.mockImplementation((command: string) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([mockPodcast])
          case 'get_episodes':
            return Promise.resolve(mockEpisodes)
          case 'get_usb_devices':
            return Promise.reject(
              new Error('USB detection service unavailable')
            )
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('USB Device')).toBeInTheDocument()
      })

      // Verify error state is displayed
      await waitFor(() => {
        expect(
          screen.getByText(/Failed to detect USB devices/)
        ).toBeInTheDocument()
        expect(
          screen.getByText(/USB detection service unavailable/)
        ).toBeInTheDocument()
      })
      console.log('✅ Error state displayed correctly')

      // Verify retry button is available
      const retryButton = screen.getByTestId('usb-retry-button')
      expect(retryButton).toBeInTheDocument()
      expect(retryButton).toHaveTextContent('🔄 Retry')
      console.log('✅ Retry button displayed correctly')

      console.log('🎉 USB device error handling E2E test PASSED!')
    })
  })

  describe('USB Device Integration with Episode Management', () => {
    it('should display USB devices alongside episode management workflow', async () => {
      console.log(
        '🚀 Testing USB device display during episode management E2E integration...'
      )

      render(<App />)

      // Wait for full app to load
      await waitFor(() => {
        expect(screen.getByText('PodPico')).toBeInTheDocument()
        expect(screen.getByText('USB Device')).toBeInTheDocument()
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      // Navigate to podcast and verify USB devices are still visible
      const podcastElement = screen.getByText('Test Podcast')
      fireEvent.click(podcastElement)

      await waitFor(() => {
        expect(screen.getByText('Episode 1: Introduction')).toBeInTheDocument()
      })

      // Verify USB devices are still displayed in sidebar
      expect(screen.getByText('SanDisk Ultra USB 3.0')).toBeInTheDocument()
      expect(screen.getByText('Kingston DataTraveler')).toBeInTheDocument()
      console.log('✅ USB devices remain visible during podcast navigation')

      // Select an episode and verify USB devices are still there
      const episode1 = screen.getByText('Episode 1: Introduction')
      fireEvent.click(episode1)

      await waitFor(() => {
        expect(screen.getByText('Mark as:')).toBeInTheDocument()
      })

      // Verify USB devices are still displayed
      expect(screen.getByText('SanDisk Ultra USB 3.0')).toBeInTheDocument()
      expect(screen.getByText('Kingston DataTraveler')).toBeInTheDocument()
      console.log('✅ USB devices remain visible during episode selection')

      console.log(
        '🎉 USB device integration with episode management E2E test PASSED!'
      )
    })

    it('should handle simultaneous USB detection and episode loading', async () => {
      console.log(
        '🚀 Testing simultaneous USB detection and episode loading E2E integration...'
      )

      // Add delays to both operations to test concurrency
      mockInvoke.mockImplementation((command: string) => {
        switch (command) {
          case 'get_podcasts':
            return new Promise(resolve => {
              setTimeout(() => resolve([mockPodcast]), 1000) // 1 second delay
            })
          case 'get_episodes':
            return new Promise(resolve => {
              setTimeout(() => resolve(mockEpisodes), 1500) // 1.5 second delay
            })
          case 'get_usb_devices':
            return new Promise(resolve => {
              setTimeout(() => resolve(mockUsbDevices), 800) // 0.8 second delay
            })
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      const startTime = Date.now()
      render(<App />)

      // Wait for USB devices to load first (should be fastest)
      await waitFor(
        () => {
          expect(screen.getByText('SanDisk Ultra USB 3.0')).toBeInTheDocument()
        },
        { timeout: 2000 }
      )

      const usbLoadTime = Date.now() - startTime
      console.log(`✅ USB devices loaded in ${usbLoadTime}ms`)

      // Wait for podcasts to load
      await waitFor(
        () => {
          expect(screen.getByText('Test Podcast')).toBeInTheDocument()
        },
        { timeout: 3000 }
      )

      const podcastLoadTime = Date.now() - startTime
      console.log(`✅ Podcasts loaded in ${podcastLoadTime}ms`)

      // Wait for episodes to load
      await waitFor(
        () => {
          expect(
            screen.getByText('Episode 1: Introduction')
          ).toBeInTheDocument()
        },
        { timeout: 4000 }
      )

      const episodeLoadTime = Date.now() - startTime
      console.log(`✅ Episodes loaded in ${episodeLoadTime}ms`)

      // Verify all components are working together
      expect(screen.getByText('USB Device')).toBeInTheDocument()
      expect(screen.getByText('SanDisk Ultra USB 3.0')).toBeInTheDocument()
      expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      expect(screen.getByText('Episode 1: Introduction')).toBeInTheDocument()

      console.log(
        '✅ All components loaded successfully with concurrent operations'
      )
      console.log(
        '🎉 Simultaneous USB detection and episode loading E2E test PASSED!'
      )
    })
  })
})
