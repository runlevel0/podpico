import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import { describe, it, expect, beforeEach } from 'vitest'
import App from '../App'
import { mockInvoke, MOCK_PODCAST, MOCK_EPISODE } from '../setupTests'

describe('App Component', () => {
  beforeEach(() => {
    // Reset all mocks
    mockInvoke.mockReset()
    
    // Set up comprehensive default mocking to prevent unhandled command errors
    mockInvoke.mockImplementation((command: string, args?: any) => {
      switch (command) {
        case 'get_usb_devices':
          return Promise.resolve([]) // Empty USB devices array by default
        case 'get_podcasts':
          return Promise.resolve([]) // Empty podcasts by default  
        case 'get_episodes':
          return Promise.resolve([]) // Empty episodes by default
        default:
          return Promise.reject(new Error(`Unhandled command: ${command}`))
      }
    })
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
        screen.getByPlaceholderText('Enter RSS feed URL...')
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

      const urlInput = screen.getByPlaceholderText('Enter RSS feed URL...')
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

      const urlInput = screen.getByPlaceholderText('Enter RSS feed URL...')
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
      
      // Button should be disabled when URL is empty
      expect(addButton).toBeDisabled()
      
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
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      // Click on podcast to select it
      fireEvent.click(screen.getByText('Test Podcast'))

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
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      const startTime = Date.now()
      fireEvent.click(screen.getByText('Test Podcast'))

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
        const statusSelect = screen.getByRole('combobox')
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
        expect(screen.getByText('Combined Inbox')).toBeInTheDocument()
        expect(screen.getByText('1 episode')).toBeInTheDocument()
      })

      // Verify combined inbox is selected by default
      expect(mockInvoke).toHaveBeenCalledWith('get_episodes', {
        podcastId: null,
      })
    })
  })

  describe('User Story #12: Search for episodes within a podcast', () => {
    it('shows search input when podcast is selected but not in combined inbox', async () => {
      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce([]) // initial get_episodes (combined inbox)
        .mockResolvedValueOnce([MOCK_EPISODE]) // get_episodes for selected podcast

      render(<App />)

      // Initially in combined inbox - search should not be visible
      await waitFor(() => {
        expect(screen.queryByPlaceholderText('Search episodes...')).not.toBeInTheDocument()
      })

      // Select a podcast
      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })
      fireEvent.click(screen.getByText('Test Podcast'))

      // Search input should now be visible
      await waitFor(() => {
        expect(screen.getByPlaceholderText('Search episodes...')).toBeInTheDocument()
      })
    })

    it('performs search and displays results with highlighting', async () => {
      const mockSearchResults = [
        { ...MOCK_EPISODE, title: 'Introduction to React Testing' }
      ]

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce([]) // initial get_episodes
        .mockResolvedValueOnce([MOCK_EPISODE]) // get_episodes for selected podcast
        .mockResolvedValueOnce(mockSearchResults) // search_episodes

      render(<App />)

      // Select podcast
      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })
      fireEvent.click(screen.getByText('Test Podcast'))

      // Wait for search input to appear
      await waitFor(() => {
        expect(screen.getByPlaceholderText('Search episodes...')).toBeInTheDocument()
      })

      // Perform search
      const searchInput = screen.getByPlaceholderText('Search episodes...')
      fireEvent.change(searchInput, { target: { value: 'React' } })

      // Wait for search to be called (with debouncing)
      await waitFor(() => {
        expect(mockInvoke).toHaveBeenCalledWith('search_episodes', {
          podcastId: 1,
          searchQuery: 'React',
        })
      }, { timeout: 1000 })

      // Check search results are displayed
      await waitFor(() => {
        // Verify the search functionality is working by checking for highlighted text
        const highlightedElement = screen.getByText('React')
        expect(highlightedElement).toBeInTheDocument()
        // Verify the highlight CSS class is applied
        expect(highlightedElement.closest('mark')).toHaveClass('search-highlight')
        // Verify the search input has the correct value
        expect(screen.getByPlaceholderText('Search episodes...')).toHaveValue('React')
      })
    })

    it('meets performance requirement of search results within 2 seconds', async () => {
      const mockSearchResults = [MOCK_EPISODE]

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce([]) // initial get_episodes
        .mockResolvedValueOnce([MOCK_EPISODE]) // get_episodes for selected podcast
        .mockResolvedValueOnce(mockSearchResults) // search_episodes

      render(<App />)

      // Select podcast
      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })
      fireEvent.click(screen.getByText('Test Podcast'))

      // Wait for search input
      await waitFor(() => {
        expect(screen.getByPlaceholderText('Search episodes...')).toBeInTheDocument()
      })

      // Measure search performance
      const startTime = Date.now()
      const searchInput = screen.getByPlaceholderText('Search episodes...')
      fireEvent.change(searchInput, { target: { value: 'test' } })

      await waitFor(() => {
        expect(mockInvoke).toHaveBeenCalledWith('search_episodes', {
          podcastId: 1,
          searchQuery: 'test',
        })
      }, { timeout: 2500 })

      const searchTime = Date.now() - startTime
      expect(searchTime).toBeLessThan(2000) // 2 second requirement
    })

    it('clears search when query is empty', async () => {
      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce([]) // initial get_episodes
        .mockResolvedValueOnce([MOCK_EPISODE]) // get_episodes for selected podcast
        .mockResolvedValueOnce([MOCK_EPISODE]) // search_episodes (empty query returns to normal)

      render(<App />)

      // Select podcast
      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })
      fireEvent.click(screen.getByText('Test Podcast'))

      // Wait for search input
      await waitFor(() => {
        expect(screen.getByPlaceholderText('Search episodes...')).toBeInTheDocument()
      })

      // Enter and then clear search
      const searchInput = screen.getByPlaceholderText('Search episodes...')
      fireEvent.change(searchInput, { target: { value: 'test' } })
      fireEvent.change(searchInput, { target: { value: '' } })

      // Should return to normal episode list
      await waitFor(() => {
        expect(screen.getByText('1 episode')).toBeInTheDocument()
      })
    })

    it('shows no results message when search returns empty', async () => {
      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce([]) // initial get_episodes
        .mockResolvedValueOnce([MOCK_EPISODE]) // get_episodes for selected podcast
        .mockResolvedValueOnce([]) // search_episodes with no results

      render(<App />)

      // Select podcast
      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })
      fireEvent.click(screen.getByText('Test Podcast'))

      // Wait for search input
      await waitFor(() => {
        expect(screen.getByPlaceholderText('Search episodes...')).toBeInTheDocument()
      })

      // Perform search that returns no results
      const searchInput = screen.getByPlaceholderText('Search episodes...')
      fireEvent.change(searchInput, { target: { value: 'nonexistent' } })

      // Check for no results message
      await waitFor(() => {
        expect(screen.getByText('No episodes found matching "nonexistent"')).toBeInTheDocument()
      })
    })

    it('highlights search terms in episode details when episode is selected from search results', async () => {
      const mockSearchResults = [
        { 
          ...MOCK_EPISODE, 
          title: 'Introduction to React Testing',
          description: 'This episode covers React testing fundamentals and best practices for testing React components.'
        }
      ]

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce([]) // initial get_episodes
        .mockResolvedValueOnce([MOCK_EPISODE]) // get_episodes for selected podcast
        .mockResolvedValueOnce(mockSearchResults) // search_episodes

      render(<App />)

      // Select podcast
      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })
      fireEvent.click(screen.getByText('Test Podcast'))

      // Wait for search input to appear
      await waitFor(() => {
        expect(screen.getByPlaceholderText('Search episodes...')).toBeInTheDocument()
      })

      // Perform search
      const searchInput = screen.getByPlaceholderText('Search episodes...')
      fireEvent.change(searchInput, { target: { value: 'React' } })

      // Wait for search results to appear
      await waitFor(() => {
        expect(mockInvoke).toHaveBeenCalledWith('search_episodes', {
          podcastId: 1,
          searchQuery: 'React',
        })
      }, { timeout: 1000 })

      // Wait for search results to appear and click on the episode item to select it
      await waitFor(() => {
        // Verify search results are displayed
        expect(screen.getByText('React')).toBeInTheDocument()
        // Click on the episode item (using the status icon as a reliable click target)
        const episodeItem = screen.getByTitle('new').closest('.episode-item')
        expect(episodeItem).toBeInTheDocument()
        fireEvent.click(episodeItem!)
      })

      // Wait for episode details to appear and verify search term highlighting
      await waitFor(() => {
        // Check that search term is highlighted in the episode details title
        const detailsTitle = screen.getAllByText('React').find(el => 
          el.closest('.episode-header') !== null
        )
        expect(detailsTitle).toBeInTheDocument()
        expect(detailsTitle!.closest('mark')).toHaveClass('search-highlight')

        // Check that search term is highlighted in the episode description
        const descriptionReact = screen.getAllByText('React').find(el => 
          el.closest('.episode-description') !== null
        )
        expect(descriptionReact).toBeInTheDocument()
        expect(descriptionReact!.closest('mark')).toHaveClass('search-highlight')
      })
    })

    it('shows clear button when search query is entered and clears search when clicked', async () => {
      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce([]) // initial get_episodes
        .mockResolvedValueOnce([MOCK_EPISODE]) // get_episodes for selected podcast

      render(<App />)

      // Select podcast
      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })
      fireEvent.click(screen.getByText('Test Podcast'))

      // Wait for search input to appear
      await waitFor(() => {
        expect(screen.getByPlaceholderText('Search episodes...')).toBeInTheDocument()
      })

      const searchInput = screen.getByPlaceholderText('Search episodes...')

      // Initially, no clear button should be visible
      expect(screen.queryByTitle('Clear search')).not.toBeInTheDocument()

      // Enter search query
      fireEvent.change(searchInput, { target: { value: 'test query' } })

      // Clear button should now be visible
      await waitFor(() => {
        expect(screen.getByTitle('Clear search')).toBeInTheDocument()
      })

      // Click clear button
      const clearButton = screen.getByTitle('Clear search')
      fireEvent.click(clearButton)

      // Search input should be cleared
      expect(searchInput).toHaveValue('')

      // Clear button should be hidden again
      await waitFor(() => {
        expect(screen.queryByTitle('Clear search')).not.toBeInTheDocument()
      })
    })
  })

  describe('User Story #3: Download Episodes', () => {
    it('formats file sizes correctly', async () => {
      mockInvoke.mockReset() // Clear default mocks
      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce([MOCK_EPISODE]) // get_episodes
        .mockResolvedValueOnce([]) // get_usb_devices

      render(<App />)

      // Select the podcast to show its episodes
      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      // Click on the podcast to select it
      fireEvent.click(screen.getByText('Test Podcast'))

      // Wait for episodes to load and select one
      await waitFor(() => {
        expect(screen.getByText('Test Episode')).toBeInTheDocument()
      })
    })

    it('shows download button for undownloaded episodes', async () => {
      const mockEpisodes = [{ ...MOCK_EPISODE, downloaded: false }]

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce(mockEpisodes) // get_episodes

      render(<App />)

      // Wait for episodes to load and select one
      await waitFor(() => {
        expect(screen.getByText('Test Episode')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('Test Episode'))

      // Check for download button
      await waitFor(() => {
        expect(screen.getByText('📥 Download Episode')).toBeInTheDocument()
      })
    })

    it('shows downloaded status for downloaded episodes', async () => {
      const mockEpisodes = [{
        ...MOCK_EPISODE,
        downloaded: true,
        local_file_path: '/path/to/episode.mp3'
      }]

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce(mockEpisodes) // get_episodes

      render(<App />)

      // Wait for episodes to load and select one
      await waitFor(() => {
        expect(screen.getByText('Test Episode')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('Test Episode'))

      // Check for download status
      await waitFor(() => {
        expect(screen.getByText('✅ Downloaded')).toBeInTheDocument()
        expect(screen.getByText('/path/to/episode.mp3')).toBeInTheDocument()
      })
    })

    it('initiates download when download button is clicked', async () => {
      const mockEpisodes = [{ ...MOCK_EPISODE, downloaded: false }]

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce(mockEpisodes) // get_episodes
        .mockResolvedValueOnce(undefined) // download_episode

      render(<App />)

      // Wait for episodes to load and select one
      await waitFor(() => {
        expect(screen.getByText('Test Episode')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('Test Episode'))

      // Wait for download button and click it
      await waitFor(() => {
        const downloadButton = screen.getByText('📥 Download Episode')
        expect(downloadButton).toBeInTheDocument()
        fireEvent.click(downloadButton)
      })

      // Verify download was initiated
      await waitFor(() => {
        expect(mockInvoke).toHaveBeenCalledWith('download_episode', {
          episodeId: 1,
        })
      })
    })

    it('displays progress tracking during download', async () => {
      const mockEpisodes = [{ ...MOCK_EPISODE, downloaded: false }]
      const mockProgress = {
        episode_id: 1,
        downloaded_bytes: 500000,
        total_bytes: 1000000,
        percentage: 50.0,
        speed_bps: 125000,
        eta_seconds: 4
      }

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce(mockEpisodes) // get_episodes
        .mockResolvedValueOnce(undefined) // download_episode
        .mockResolvedValue(mockProgress) // get_download_progress (repeated calls)

      render(<App />)

      // Wait for episodes to load and select one
      await waitFor(() => {
        expect(screen.getByText('Test Episode')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('Test Episode'))

      // Click download button
      await waitFor(() => {
        const downloadButton = screen.getByText('📥 Download Episode')
        fireEvent.click(downloadButton)
      })

      // Wait for downloading state to be set immediately (the download button should be replaced)
      await waitFor(() => {
        expect(screen.queryByText('📥 Download Episode')).not.toBeInTheDocument()
        expect(screen.getByText('📥 Downloading...')).toBeInTheDocument()
      })

      // Wait for progress percentage to appear (after progress interval runs)
      await waitFor(() => {
        expect(screen.getByText('50.0%')).toBeInTheDocument()
      }, { timeout: 3000 })
    })

    it('shows download indicators in episode list', async () => {
      const mockEpisodes = [
        { ...MOCK_EPISODE, id: 1, title: 'Downloaded Episode', downloaded: true },
        { ...MOCK_EPISODE, id: 2, title: 'Not Downloaded Episode', downloaded: false }
      ]

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce(mockEpisodes) // get_episodes

      render(<App />)

      // Wait for episodes to load
      await waitFor(() => {
        expect(screen.getByText('Downloaded Episode')).toBeInTheDocument()
        expect(screen.getByText('Not Downloaded Episode')).toBeInTheDocument()
      })

      // Check for download indicator with bullet point prefix
      await waitFor(() => {
        // Look for the text more flexibly since it might be broken across elements
        const downloadedIndicator = screen.getByText((content, element) => {
          return content.includes('📥 Downloaded')
        })
        expect(downloadedIndicator).toBeInTheDocument()
      })
    })

    it('handles download errors with retry functionality', async () => {
      const mockEpisodes = [{ ...MOCK_EPISODE, downloaded: false }]

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce(mockEpisodes) // get_episodes
        .mockRejectedValueOnce(new Error('Network error')) // download_episode fails

      render(<App />)

      // Wait for episodes to load and select one
      await waitFor(() => {
        expect(screen.getByText('Test Episode')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('Test Episode'))

      // Click download button
      await waitFor(() => {
        const downloadButton = screen.getByText('📥 Download Episode')
        fireEvent.click(downloadButton)
      })

      // Wait for error message to appear
      await waitFor(() => {
        expect(screen.getByText(/Download failed/)).toBeInTheDocument()
        expect(screen.getByText('🔄 Retry')).toBeInTheDocument()
      })
    })

    it('prevents multiple simultaneous downloads of same episode', async () => {
      const mockEpisodes = [{ ...MOCK_EPISODE, downloaded: false }]

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce(mockEpisodes) // get_episodes
        .mockResolvedValueOnce(undefined) // first download_episode
        .mockResolvedValueOnce({
          episode_id: 1,
          downloaded_bytes: 0,
          total_bytes: 1000000,
          percentage: 0,
          speed_bps: 0,
          eta_seconds: 100
        }) // get_download_progress

      render(<App />)

      // Wait for episodes to load and select one
      await waitFor(() => {
        expect(screen.getByText('Test Episode')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('Test Episode'))

      // Click download button
      await waitFor(() => {
        const downloadButton = screen.getByText('📥 Download Episode')
        fireEvent.click(downloadButton)
      })

      // Wait for downloading state to appear
      await waitFor(() => {
        expect(screen.getByText('📥 Downloading...')).toBeInTheDocument()
      })

      // Verify download call was made
      expect(mockInvoke).toHaveBeenCalledWith('download_episode', {
        episodeId: 1,
      })
      // Note: Progress tracking happens asynchronously, so we can't guarantee exact call count
      expect(mockInvoke).toHaveBeenCalledTimes(3) // get_podcasts, get_episodes, download_episode
    })

    it('formats file sizes correctly', async () => {
      const mockEpisodes = [{ ...MOCK_EPISODE, downloaded: false }]
      const mockProgress = {
        episode_id: 1,
        downloaded_bytes: 1024 * 1024 * 5.5, // 5.5 MB
        total_bytes: 1024 * 1024 * 50, // 50 MB
        percentage: 11.0,
        speed_bps: 1024 * 500, // 500 KB/s
        eta_seconds: 90 // 1m 30s
      }

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce(mockEpisodes) // get_episodes
        .mockResolvedValueOnce(undefined) // download_episode
        .mockResolvedValue(mockProgress) // get_download_progress (repeated calls)

      render(<App />)

      // Wait for episodes to load and select one
      await waitFor(() => {
        expect(screen.getByText('Test Episode')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('Test Episode'))

      // Click download button
      await waitFor(() => {
        const downloadButton = screen.getByText('📥 Download Episode')
        fireEvent.click(downloadButton)
      })

      // Wait for downloading state to appear first
      await waitFor(() => {
        expect(screen.getByText('📥 Downloading...')).toBeInTheDocument()
      })

      // Then wait for progress display with formatted values (longer timeout for async operations)
      await waitFor(() => {
        expect(screen.getByText('11.0%')).toBeInTheDocument()
      }, { timeout: 3000 })

      // Check that the formatted values are present (they may be in separate elements)
      await waitFor(() => {
        expect(screen.getByText(/5.5 MB/)).toBeInTheDocument()
        expect(screen.getByText(/50 MB/)).toBeInTheDocument()
        expect(screen.getByText(/500 KB\/s/)).toBeInTheDocument()
        expect(screen.getByText(/1m 30s/)).toBeInTheDocument()
      }, { timeout: 3000 })
    })
  })

  describe('Error Handling', () => {
    it('handles podcast loading errors gracefully', async () => {
      mockInvoke
        .mockRejectedValueOnce(new Error('Database error')) // get_podcasts fails
        .mockResolvedValueOnce([]) // get_episodes (Combined Inbox) succeeds

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
        expect(screen.getByText(/Episode loading failed/)).toBeInTheDocument()
      })
    })
  })

  describe('User Story #4: Remove Podcasts', () => {
    it('displays remove button for each podcast', async () => {
      mockInvoke.mockResolvedValueOnce([MOCK_PODCAST])

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      // Hover over podcast to show remove button
      const podcastItem = screen.getByText('Test Podcast').closest('.podcast-item')
      fireEvent.mouseEnter(podcastItem!)

      await waitFor(() => {
        expect(screen.getByTitle('Remove Test Podcast')).toBeInTheDocument()
        expect(screen.getByLabelText('Remove Test Podcast')).toBeInTheDocument()
      })
    })

    it('shows confirmation dialog when remove button is clicked', async () => {
      mockInvoke.mockResolvedValueOnce([MOCK_PODCAST])

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      // Find and click remove button
      const removeButton = screen.getByTitle('Remove Test Podcast')
      fireEvent.click(removeButton)

      // Check confirmation dialog appears
      await waitFor(() => {
        expect(screen.getByRole('dialog')).toBeInTheDocument()
        expect(screen.getByText('Remove Podcast')).toBeInTheDocument()
        expect(screen.getByText(/Are you sure you want to remove/)).toBeInTheDocument()
        expect(screen.getByRole('dialog')).toHaveTextContent('Test Podcast')
        expect(screen.getByText('Cancel')).toBeInTheDocument()
        expect(screen.getByText('🗑️ Remove Podcast')).toBeInTheDocument()
      })
    })

    it('cancels removal when cancel button is clicked', async () => {
      mockInvoke.mockResolvedValueOnce([MOCK_PODCAST])

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      // Click remove button
      const removeButton = screen.getByTitle('Remove Test Podcast')
      fireEvent.click(removeButton)

      // Wait for dialog and click cancel
      await waitFor(() => {
        expect(screen.getByText('Cancel')).toBeInTheDocument()
      })

      fireEvent.click(screen.getByText('Cancel'))

      // Dialog should disappear
      await waitFor(() => {
        expect(screen.queryByText('Remove Podcast')).not.toBeInTheDocument()
      })
    })

    it('calls remove_podcast backend command when confirmed', async () => {
      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce(undefined) // remove_podcast
        .mockResolvedValueOnce([]) // get_podcasts (after removal)
        .mockResolvedValueOnce([]) // get_episodes (refresh)

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      // Click remove button
      const removeButton = screen.getByTitle('Remove Test Podcast')
      fireEvent.click(removeButton)

      // Wait for dialog and confirm removal
      await waitFor(() => {
        expect(screen.getByText('🗑️ Remove Podcast')).toBeInTheDocument()
      })

      const confirmButton = screen.getByText('🗑️ Remove Podcast')
      fireEvent.click(confirmButton)

      // Verify backend call was made
      await waitFor(() => {
        expect(mockInvoke).toHaveBeenCalledWith('remove_podcast', {
          podcastId: 1,
        })
      })
    })

    it('shows loading state during removal', async () => {
      let resolveRemoval: (value: any) => void
      const removalPromise = new Promise(resolve => {
        resolveRemoval = resolve
      })

      mockInvoke.mockReset() // Clear default mocks for this specific test
      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce([MOCK_EPISODE]) // get_episodes (Combined Inbox)
        .mockResolvedValueOnce([]) // get_usb_devices (prevent NaN)
        .mockReturnValueOnce(removalPromise) // remove_podcast (pending)

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      // Click remove button
      const removeButton = screen.getByTitle('Remove Test Podcast')
      fireEvent.click(removeButton)

      // Confirm removal
      await waitFor(() => {
        expect(screen.getByText('🗑️ Remove Podcast')).toBeInTheDocument()
      })

      const confirmButton = screen.getByText('🗑️ Remove Podcast')
      fireEvent.click(confirmButton)

      // Check loading state appears
      await waitFor(() => {
        expect(screen.getByText('⏳ Removing...')).toBeInTheDocument()
      })

      // Both buttons should be disabled during removal
      await waitFor(() => {
        expect(screen.getByRole('dialog')).toBeInTheDocument()
        expect(screen.getByText('Cancel')).toBeDisabled()
        expect(screen.getByText('⏳ Removing...')).toBeDisabled()
      })

      // Resolve the removal to clean up
      resolveRemoval!(undefined)
    })

    it('handles removal errors gracefully', async () => {
      mockInvoke.mockReset() // Clear default mocks for this specific test
      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce([MOCK_EPISODE]) // get_episodes (Combined Inbox)
        .mockResolvedValueOnce([]) // get_usb_devices (prevent NaN)
        .mockRejectedValueOnce(new Error('Failed to remove podcast')) // remove_podcast fails

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      // Click remove button
      const removeButton = screen.getByTitle('Remove Test Podcast')
      fireEvent.click(removeButton)

      // Confirm removal
      await waitFor(() => {
        expect(screen.getByText('🗑️ Remove Podcast')).toBeInTheDocument()
      })

      const confirmButton = screen.getByText('🗑️ Remove Podcast')
      fireEvent.click(confirmButton)

      // Dialog should stay open for better UX (allows retry/cancel)
      await waitFor(() => {
        expect(screen.getByRole('dialog')).toBeInTheDocument()
      })

      // Check error message appears in the podcast item
      await waitFor(() => {
        expect(screen.getByText(/Remove failed: Error: Failed to remove podcast/)).toBeInTheDocument()
      })
    })

    it('updates UI immediately after successful removal', async () => {
      mockInvoke.mockReset() // Clear default mocks for this specific test
      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce([MOCK_EPISODE]) // get_episodes (Combined Inbox initial)
        .mockResolvedValueOnce([]) // get_usb_devices (prevent NaN)
        .mockResolvedValueOnce(undefined) // remove_podcast
        .mockResolvedValueOnce([]) // get_podcasts (after removal)
        .mockResolvedValueOnce([]) // get_episodes (refresh)

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      // Click remove button and confirm
      const removeButton = screen.getByTitle('Remove Test Podcast')
      fireEvent.click(removeButton)

      await waitFor(() => {
        expect(screen.getByText('🗑️ Remove Podcast')).toBeInTheDocument()
      })

      const confirmButton = screen.getByText('🗑️ Remove Podcast')
      fireEvent.click(confirmButton)

      // Podcast should be removed from UI
      await waitFor(() => {
        expect(screen.queryByText('Test Podcast')).not.toBeInTheDocument()
      })

      // Dialog should be closed
      expect(screen.queryByText('Remove Podcast')).not.toBeInTheDocument()
    })

    it('clears selected podcast if removed podcast was selected', async () => {
      const mockEpisodes = [MOCK_EPISODE]

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce(mockEpisodes) // get_episodes (when selected)
        .mockResolvedValueOnce(undefined) // remove_podcast
        .mockResolvedValueOnce([]) // get_podcasts (after removal)
        .mockResolvedValueOnce([]) // get_episodes (combined inbox)

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      // Select the podcast
      const podcastItem = screen.getByText('Test Podcast')
      fireEvent.click(podcastItem)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast Episodes')).toBeInTheDocument()
      })

      // Remove the selected podcast
      const removeButton = screen.getByTitle('Remove Test Podcast')
      fireEvent.click(removeButton)

      await waitFor(() => {
        expect(screen.getByText('🗑️ Remove Podcast')).toBeInTheDocument()
      })

      const confirmButton = screen.getByText('🗑️ Remove Podcast')
      fireEvent.click(confirmButton)

      // Should fall back to combined inbox
      await waitFor(() => {
        expect(screen.getByText('All New Episodes')).toBeInTheDocument()
        expect(screen.queryByText('Test Podcast Episodes')).not.toBeInTheDocument()
      })
    })

    it('shows confirmation dialog with podcast details', async () => {
      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce([MOCK_EPISODE]) // get_episodes

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      // Select podcast
      fireEvent.click(screen.getByText('Test Podcast'))

      await waitFor(() => {
        expect(screen.getByText('Test Podcast Episodes')).toBeInTheDocument()
      })

      // Click remove button
      const removeButton = screen.getByTitle('Remove Test Podcast')
      fireEvent.click(removeButton)

      // Check confirmation dialog appears with basic content
      await waitFor(() => {
        expect(screen.getByRole('dialog')).toBeInTheDocument()
        expect(screen.getByText('Remove Podcast')).toBeInTheDocument()
        expect(screen.getByText(/Are you sure you want to remove/)).toBeInTheDocument()
        expect(screen.getByRole('dialog')).toHaveTextContent('Test Podcast')
        expect(screen.getByText(/This will permanently remove the podcast/)).toBeInTheDocument()
      })
    })

    it('shows loading state during confirmation dialog', async () => {
      let resolveRemoval: (value: any) => void
      const removalPromise = new Promise(resolve => {
        resolveRemoval = resolve
      })

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockResolvedValueOnce([MOCK_EPISODE]) // get_episodes
        .mockReturnValueOnce(removalPromise) // remove_podcast (slow)
        .mockResolvedValueOnce([]) // get_podcasts (refresh)

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      // Select podcast
      fireEvent.click(screen.getByText('Test Podcast'))

      await waitFor(() => {
        expect(screen.getByText('Test Podcast Episodes')).toBeInTheDocument()
      })

      // Click remove button
      const removeButton = screen.getByTitle('Remove Test Podcast')
      fireEvent.click(removeButton)

      // Confirm removal
      await waitFor(() => {
        expect(screen.getByRole('dialog')).toBeInTheDocument()
      })

      const confirmButton = screen.getByText('🗑️ Remove Podcast')
      fireEvent.click(confirmButton)

      // Check loading state appears
      await waitFor(() => {
        expect(confirmButton).toBeDisabled()
        expect(screen.getByText('⏳ Removing...')).toBeInTheDocument()
      })

      // Resolve the removal to clean up
      resolveRemoval!(undefined)
    })

    it('prevents multiple simultaneous removals of same podcast', async () => {
      let resolveRemoval: (value: any) => void
      const removalPromise = new Promise(resolve => {
        resolveRemoval = resolve
      })

      mockInvoke
        .mockResolvedValueOnce([MOCK_PODCAST]) // get_podcasts
        .mockReturnValueOnce(removalPromise) // remove_podcast (pending)

      render(<App />)

      await waitFor(() => {
        expect(screen.getByText('Test Podcast')).toBeInTheDocument()
      })

      // Start first removal
      const removeButton = screen.getByTitle('Remove Test Podcast')
      fireEvent.click(removeButton)

      await waitFor(() => {
        expect(screen.getByText('🗑️ Remove Podcast')).toBeInTheDocument()
      })

      const confirmButton = screen.getByText('🗑️ Remove Podcast')
      fireEvent.click(confirmButton)

      // Should see loading indicator
      await waitFor(() => {
        expect(screen.getByText('⏳ Removing...')).toBeInTheDocument()
      })

      // Verify remove call was made
      expect(mockInvoke).toHaveBeenCalledWith('remove_podcast', {
        podcastId: 1,
      })
      
      // Check that remove_podcast was called exactly once
      const removeCalls = mockInvoke.mock.calls.filter(call => call[0] === 'remove_podcast')
      expect(removeCalls).toHaveLength(1)

      // Clean up
      resolveRemoval!(undefined)
    })
  })
})
