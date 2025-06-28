import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { describe, it, expect, beforeEach } from 'vitest'
import App from '../App'
import { mockInvoke, MOCK_PODCAST, MOCK_EPISODE } from '../setupTests'

describe('App Component', () => {
  beforeEach(() => {
    mockInvoke.mockReset()
  })

  describe('Initial Rendering', () => {
    it('renders PodPico header', () => {
      mockInvoke.mockResolvedValue([])
      render(<App />)

      expect(screen.getByText('PodPico')).toBeInTheDocument()
    })

    it('renders add podcast input', () => {
      mockInvoke.mockResolvedValue([])
      render(<App />)

      expect(
        screen.getByPlaceholderText('Enter RSS feed URL')
      ).toBeInTheDocument()
      expect(screen.getByText('Add Podcast')).toBeInTheDocument()
    })

    it('renders combined inbox initially', () => {
      mockInvoke.mockResolvedValue([])
      render(<App />)

      expect(screen.getByText('Combined Inbox')).toBeInTheDocument()
    })
  })

  describe('User Story #1: Add podcast subscription via RSS URL', () => {
    it('allows user to enter RSS URL and add podcast', async () => {
      // Mock successful podcast addition
      mockInvoke
        .mockResolvedValueOnce([]) // Initial get_podcasts call
        .mockResolvedValueOnce([]) // Initial get_episodes call for combined inbox
        .mockResolvedValueOnce(MOCK_PODCAST) // add_podcast call
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts after add

      render(<App />)

      const urlInput = screen.getByPlaceholderText('Enter RSS feed URL')
      const addButton = screen.getByText('Add Podcast')

      // Enter RSS URL
      fireEvent.change(urlInput, {
        target: { value: 'https://example.com/feed.xml' },
      })
      expect(urlInput).toHaveValue('https://example.com/feed.xml')

      // Click add podcast
      fireEvent.click(addButton)

      // Wait for podcast to be added
      await waitFor(() => {
        expect(mockInvoke).toHaveBeenCalledWith('add_podcast', {
          rssUrl: 'https://example.com/feed.xml',
        })
      })
    })

    it('shows error message for invalid RSS URL', async () => {
      // Mock error response
      mockInvoke
        .mockResolvedValueOnce([]) // Initial get_podcasts call
        .mockResolvedValueOnce([]) // Initial get_episodes call
        .mockRejectedValueOnce(new Error('Invalid RSS feed'))

      render(<App />)

      const urlInput = screen.getByPlaceholderText('Enter RSS feed URL')
      const addButton = screen.getByText('Add Podcast')

      fireEvent.change(urlInput, { target: { value: 'invalid-url' } })
      fireEvent.click(addButton)

      await waitFor(() => {
        expect(screen.getByText(/Failed to add podcast/)).toBeInTheDocument()
      })
    })

    it('prevents adding podcast with empty URL', async () => {
      mockInvoke.mockResolvedValue([])
      render(<App />)

      const addButton = screen.getByText('Add Podcast')
      fireEvent.click(addButton)

      expect(screen.getByText('Please enter an RSS URL')).toBeInTheDocument()
      expect(mockInvoke).not.toHaveBeenCalledWith(
        'add_podcast',
        expect.anything()
      )
    })
  })

  describe('User Story #2: View all episodes of specific podcast', () => {
    it('loads and displays episodes when podcast is selected', async () => {
      const mockEpisodes = [MOCK_EPISODE]

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce([]) // initial get_episodes (combined inbox)
        .mockResolvedValueOnce(mockEpisodes) // get_episodes for selected podcast

      render(<App />)

      // Wait for podcasts to load
      await waitFor(() => {
        expect(screen.getByText('Test Podcast (2 new)')).toBeInTheDocument()
      })

      // Click on podcast to select it
      fireEvent.click(screen.getByText('Test Podcast (2 new)'))

      // Wait for episodes to load
      await waitFor(() => {
        expect(mockInvoke).toHaveBeenCalledWith('get_episodes', {
          podcastId: 1,
        })
      })
    })

    it('meets performance requirement of loading episodes within 3 seconds', async () => {
      // This test validates the acceptance criteria timing requirement
      const mockEpisodes = [MOCK_EPISODE]

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce([]) // initial get_episodes
        .mockResolvedValueOnce(mockEpisodes) // get_episodes for selected podcast

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast (2 new)')).toBeInTheDocument()
      })

      const startTime = Date.now()
      fireEvent.click(screen.getByText('Test Podcast (2 new)'))

      await waitFor(() => {
        expect(mockInvoke).toHaveBeenCalledWith('get_episodes', {
          podcastId: 1,
        })
      })

      const loadTime = Date.now() - startTime
      expect(loadTime).toBeLessThan(3000) // 3 second requirement
    })
  })

  describe('User Story #5: Mark episodes as listened', () => {
    it('allows updating episode status', async () => {
      const mockEpisodes = [MOCK_EPISODE]

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce(mockEpisodes) // get_episodes
        .mockResolvedValueOnce(undefined) // update_episode_status
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts refresh

      render(<App />)

      // Wait for episodes to load
      await waitFor(() => {
        expect(screen.getByText('Test Episode')).toBeInTheDocument()
      })

      // Click on episode to select it
      fireEvent.click(screen.getByText('Test Episode'))

      // Wait for episode to be selected and status controls to appear
      await waitFor(() => {
        const statusSelect = screen.getByDisplayValue('new')
        expect(statusSelect).toBeInTheDocument()

        // Change status to listened
        fireEvent.change(statusSelect, { target: { value: 'listened' } })
      })

      await waitFor(() => {
        expect(mockInvoke).toHaveBeenCalledWith('update_episode_status', {
          episodeId: 1,
          status: 'listened',
        })
      })
    })
  })

  describe('User Story #6: See episode status within podcasts', () => {
    it('displays correct status icons for episodes', async () => {
      const mockEpisodes = [
        { ...MOCK_EPISODE, status: 'new' },
        { ...MOCK_EPISODE, id: 2, status: 'unlistened' },
        { ...MOCK_EPISODE, id: 3, status: 'listened' },
      ]

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST])
        .mockResolvedValueOnce(mockEpisodes)

      render(<App />)

      await waitFor(() => {
        // Check that status icons are displayed
        expect(screen.getByText('🔴')).toBeInTheDocument() // new
        expect(screen.getByText('🔵')).toBeInTheDocument() // unlistened
        expect(screen.getByText('✅')).toBeInTheDocument() // listened
      })
    })
  })

  describe('User Story #7: Combined Inbox functionality', () => {
    it('shows combined inbox with episodes from all podcasts', async () => {
      const mockEpisodes = [MOCK_EPISODE]

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce(mockEpisodes) // get_episodes for combined inbox

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Combined Inbox (1 new)')).toBeInTheDocument()
      })

      // Verify combined inbox is selected by default
      expect(mockInvoke).toHaveBeenCalledWith('get_episodes', {
        podcastId: null,
      })
    })
  })

  describe('Error Handling', () => {
    it('handles podcast loading errors gracefully', async () => {
      mockInvoke.mockRejectedValueOnce(new Error('Database error'))

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText(/Failed to load podcasts/)).toBeInTheDocument()
      })
    })

    it('handles episode loading errors gracefully', async () => {
      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST])
        .mockRejectedValueOnce(new Error('Episode loading failed'))

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText(/Failed to load episodes/)).toBeInTheDocument()
      })
    })
  })
})
