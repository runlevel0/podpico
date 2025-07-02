import React from 'react'
import { render, screen, waitFor, act } from '@testing-library/react'
import { vi, describe, it, expect, beforeEach, afterEach } from 'vitest'
import App from '../App'

// Mock Tauri API - must be outside of any other code blocks
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

// Import the mocked invoke after the mock setup
import { invoke } from '@tauri-apps/api/core'
const mockInvoke = vi.mocked(invoke)

describe('User Story #8: USB Device Management Frontend', () => {
  beforeEach(() => {
    vi.clearAllMocks()
    
    // Default mock responses for basic app functionality
    mockInvoke.mockImplementation((command: string) => {
      switch (command) {
        case 'get_podcasts':
          return Promise.resolve([])
        case 'get_episodes':
          return Promise.resolve([])
        case 'get_usb_devices':
          return Promise.resolve([])
        default:
          return Promise.reject(new Error(`Unhandled command: ${command}`))
      }
    })
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  describe('USB Device Section Display', () => {
    it('should display USB device section in left sidebar', async () => {
      render(<App />)
      
      await waitFor(() => {
        // Look for USB device section in sidebar
        expect(screen.getByText('USB Device')).toBeInTheDocument()
      })
    })

    it('should show "No device connected" message when no USB device is found', async () => {
      // Mock empty USB devices response
      mockInvoke.mockImplementation((command: string) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([])
          case 'get_episodes':
            return Promise.resolve([])
          case 'get_usb_devices':
            return Promise.resolve([]) // No USB devices
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      render(<App />)
      
      await waitFor(() => {
        expect(screen.getByText('📱 No device connected')).toBeInTheDocument()
      })
    })

    it('should call get_usb_devices backend command on component mount', async () => {
      render(<App />)
      
      await waitFor(() => {
        expect(mockInvoke).toHaveBeenCalledWith('get_usb_devices')
      })
    })
  })

  describe('Connected USB Device Display', () => {
    const mockUsbDevice = {
      id: 'test-device-1',
      name: 'Test USB Drive',
      path: '/media/test-device',
      total_space: 8589934592, // 8GB in bytes
      available_space: 4294967296, // 4GB in bytes
      is_connected: true
    }

    it('should display connected USB device information', async () => {
      mockInvoke.mockImplementation((command: string) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([])
          case 'get_episodes':
            return Promise.resolve([])
          case 'get_usb_devices':
            return Promise.resolve([mockUsbDevice])
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      render(<App />)
      
      await waitFor(() => {
        expect(screen.getByText('Test USB Drive')).toBeInTheDocument()
        expect(screen.getByText('📱')).toBeInTheDocument() // USB device icon
      })
    })

    it('should display storage capacity with numerical indicators (MB/GB)', async () => {
      mockInvoke.mockImplementation((command: string) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([])
          case 'get_episodes':
            return Promise.resolve([])
          case 'get_usb_devices':
            return Promise.resolve([mockUsbDevice])
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      render(<App />)
      
      await waitFor(() => {
        // Check for storage capacity display - should show 4GB / 8GB
        expect(screen.getByText(/4\.0 GB/)).toBeInTheDocument()
        expect(screen.getByText(/8\.0 GB/)).toBeInTheDocument()
        expect(screen.getByText(/available/i)).toBeInTheDocument()
      })
    })

    it('should display storage capacity with visual progress bar indicator', async () => {
      mockInvoke.mockImplementation((command: string) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([])
          case 'get_episodes':
            return Promise.resolve([])
          case 'get_usb_devices':
            return Promise.resolve([mockUsbDevice])
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      render(<App />)
      
      await waitFor(() => {
        // Look for progress bar element
        const progressBar = screen.getByRole('progressbar')
        expect(progressBar).toBeInTheDocument()
        
        // Should show 50% usage (4GB used out of 8GB total)
        expect(progressBar).toHaveAttribute('aria-valuenow', '50')
        expect(progressBar).toHaveAttribute('aria-valuemin', '0')
        expect(progressBar).toHaveAttribute('aria-valuemax', '100')
      })
    })
  })

  describe('USB Device Detection Performance', () => {
    it('should detect and display USB device within 5 seconds (User Story #8 acceptance criteria)', async () => {
      const mockUsbDevice = {
        id: 'test-device-1',
        name: 'Test USB Drive',
        path: '/media/test-device',
        total_space: 8589934592,
        available_space: 4294967296,
        is_connected: true
      }

      // Add delay to simulate detection time but keep under 5 seconds
      mockInvoke.mockImplementation((command: string) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([])
          case 'get_episodes':
            return Promise.resolve([])
          case 'get_usb_devices':
            return new Promise(resolve => {
              setTimeout(() => resolve([mockUsbDevice]), 2000) // 2 second delay
            })
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      const startTime = Date.now()
      render(<App />)
      
      await waitFor(() => {
        expect(screen.getByText('Test USB Drive')).toBeInTheDocument()
      }, { timeout: 5000 }) // 5 second timeout as per acceptance criteria
      
      const detectionTime = Date.now() - startTime
      expect(detectionTime).toBeLessThan(5000) // Must be under 5 seconds
    })
  })

  describe('Error Handling', () => {
    it('should handle USB device detection errors gracefully', async () => {
      mockInvoke.mockImplementation((command: string) => {
        switch (command) {
          case 'get_podcasts':
            return Promise.resolve([])
          case 'get_episodes':
            return Promise.resolve([])
          case 'get_usb_devices':
            return Promise.reject(new Error('USB detection failed'))
          default:
            return Promise.reject(new Error(`Unhandled command: ${command}`))
        }
      })

      render(<App />)
      
      await waitFor(() => {
        // Should show error state when USB detection fails
        expect(screen.getByText(/Failed to detect USB devices/i)).toBeInTheDocument()
        expect(screen.getByText(/USB detection failed/i)).toBeInTheDocument()
      })
    })
  })
}) 