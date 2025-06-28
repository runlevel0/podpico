import React, { useState, useEffect, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import './App.css'

interface Podcast {
  id: number
  name: string
  rss_url: string
  description?: string
  artwork_url?: string
  website_url?: string
  last_updated?: string
  episode_count: number
  new_episode_count: number
}

interface Episode {
  id: number
  podcast_id: number
  podcast_name: string
  title: string
  description?: string
  episode_url: string
  published_date?: string
  duration?: number
  file_size?: number
  local_file_path?: string
  status: string
  downloaded: boolean
  on_device: boolean
}

function App() {
  const [podcasts, setPodcasts] = useState<Podcast[]>([])
  const [episodes, setEpisodes] = useState<Episode[]>([])
  const [selectedPodcast, setSelectedPodcast] = useState<Podcast | null>(null)
  const [selectedEpisode, setSelectedEpisode] = useState<Episode | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  const [rssUrl, setRssUrl] = useState('')
  const [addingPodcast, setAddingPodcast] = useState(false)
  
  // User Story #12: Search for episodes within a podcast
  const [searchQuery, setSearchQuery] = useState('')
  const [searchResults, setSearchResults] = useState<Episode[]>([])
  const [isSearching, setIsSearching] = useState(false)
  const [isSearchMode, setIsSearchMode] = useState(false)

  // Load podcasts on component mount
  useEffect(() => {
    loadPodcasts()
  }, [])

  // Load episodes when podcast selection changes
  useEffect(() => {
    if (selectedPodcast) {
      loadEpisodes(selectedPodcast.id)
    } else {
      // Load all new episodes for Combined Inbox
      loadEpisodes(null)
    }
  }, [selectedPodcast])

  async function loadPodcasts() {
    try {
      const podcastList: Podcast[] = await invoke('get_podcasts')
      setPodcasts(podcastList)
    } catch (err) {
      // eslint-disable-next-line no-console
      console.error('Failed to load podcasts:', err)
      setError(`Failed to load podcasts: ${err}`)
    }
  }

  async function loadEpisodes(podcastId: number | null) {
    setLoading(true)
    try {
      // User Story #2: View all episodes of specific podcast
      // Acceptance Criteria: Episodes display within 3 seconds
      const startTime = Date.now()

      const episodeList: Episode[] = await invoke('get_episodes', {
        podcastId: podcastId,
      })

      const loadTime = Date.now() - startTime
      // Performance monitoring for User Story #2 acceptance criteria
      if (loadTime > 3000) {
        // eslint-disable-next-line no-console
        console.warn(
          `User Story #2: Load time exceeded 3 seconds (${loadTime}ms)`
        )
      }

      setEpisodes(episodeList)
      setSelectedEpisode(null) // Clear episode selection when changing podcast
    } catch (err) {
      // eslint-disable-next-line no-console
      console.error('Failed to load episodes:', err)
      setError(`Failed to load episodes: ${err}`)
    } finally {
      setLoading(false)
    }
  }

  async function addPodcast() {
    if (!rssUrl.trim()) {
      setError('Please enter an RSS URL')
      return
    }

    setAddingPodcast(true)
    setError('')

    try {
      // User Story #1: Add new podcast subscription via RSS URL
      await invoke('add_podcast', { rssUrl })

      await loadPodcasts()
      setRssUrl('')
      setError('')
    } catch (err) {
      // eslint-disable-next-line no-console
      console.error('Failed to add podcast:', err)
      setError(`Failed to add podcast: ${err}`)
    } finally {
      setAddingPodcast(false)
    }
  }

  async function updateEpisodeStatus(episodeId: number, newStatus: string) {
    try {
      // User Story #5: Mark episodes as "listened"
      // Acceptance Criteria: Status persists across sessions
      await invoke('update_episode_status', {
        episodeId,
        status: newStatus,
      })

      // Update local state immediately for responsive UI
      setEpisodes(prevEpisodes =>
        prevEpisodes.map(episode =>
          episode.id === episodeId ? { ...episode, status: newStatus } : episode
        )
      )

      // Update selected episode if it's the one being changed
      if (selectedEpisode?.id === episodeId) {
        setSelectedEpisode(prev =>
          prev ? { ...prev, status: newStatus } : null
        )
      }

      // Refresh podcasts to update episode counts
      await loadPodcasts()
    } catch (err) {
      // eslint-disable-next-line no-console
      console.error('Failed to update episode status:', err)
      setError(`Failed to update episode status: ${err}`)
    }
  }

  // User Story #12: Search for episodes within a podcast
  // Acceptance Criteria: Search results appear within 2 seconds with highlighted text
  const searchEpisodes = useCallback(async (query: string) => {
    if (!selectedPodcast) return

    setIsSearching(true)
    try {
      const startTime = Date.now()
      
      if (query.trim() === '') {
        // Clear search - return to normal episode list
        setIsSearchMode(false)
        setSearchResults([])
        await loadEpisodes(selectedPodcast.id)
      } else {
        // Perform search
        const results: Episode[] = await invoke('search_episodes', {
          podcastId: selectedPodcast.id,
          searchQuery: query,
        })

        const searchTime = Date.now() - startTime
        
        // Performance monitoring for User Story #12 acceptance criteria
        if (searchTime > 2000) {
          // eslint-disable-next-line no-console
          console.warn(
            `User Story #12: Search took ${searchTime}ms, should be under 2 seconds`
          )
        }

        setSearchResults(results)
        setIsSearchMode(true)
      }
    } catch (err) {
      // eslint-disable-next-line no-console
      console.error('Failed to search episodes:', err)
      setError(`Failed to search episodes: ${err}`)
    } finally {
      setIsSearching(false)
    }
  }, [selectedPodcast])

  // Debounced search effect for User Story #12
  // Acceptance Criteria: Search results appear within 2 seconds
  useEffect(() => {
    if (!selectedPodcast) return

    const debounceTimer = setTimeout(() => {
      searchEpisodes(searchQuery)
    }, 300) // 300ms debounce for responsive search

    return () => clearTimeout(debounceTimer)
  }, [searchQuery, selectedPodcast, searchEpisodes])

  // Clear search when podcast selection changes
  useEffect(() => {
    setSearchQuery('')
    setIsSearchMode(false)
    setSearchResults([])
  }, [selectedPodcast])

  function formatDuration(seconds?: number): string {
    if (!seconds) return 'Unknown'

    const hours = Math.floor(seconds / 3600)
    const minutes = Math.floor((seconds % 3600) / 60)
    const secs = seconds % 60

    if (hours > 0) {
      return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
    } else {
      return `${minutes}:${secs.toString().padStart(2, '0')}`
    }
  }

  function formatDate(dateStr?: string): string {
    if (!dateStr) return 'Unknown date'

    try {
      const date = new Date(dateStr)
      return date.toLocaleDateString()
    } catch {
      return dateStr
    }
  }

  function getStatusIcon(status: string): string {
    switch (status) {
      case 'new':
        return '🔴' // Red circle for new
      case 'unlistened':
        return '🔵' // Blue circle for unlistened
      case 'listened':
        return '✅' // Green checkmark for listened
      default:
        return '❓'
    }
  }

  function getTotalNewEpisodes(): number {
    return podcasts.reduce(
      (total, podcast) => total + podcast.new_episode_count,
      0
    )
  }

  // User Story #12: Highlight matching text in search results
  // Acceptance Criteria: Matching text is highlighted in episode titles/descriptions
  function highlightText(text: string, searchQuery: string): React.JSX.Element {
    if (!searchQuery.trim() || !text) {
      return <>{text}</>
    }

    const regex = new RegExp(`(${searchQuery.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi')
    const parts = text.split(regex)

    return (
      <>
        {parts.map((part, index) =>
          regex.test(part) ? (
            <mark key={index} className="search-highlight">
              {part}
            </mark>
          ) : (
            <span key={index}>{part}</span>
          )
        )}
      </>
    )
  }

  return (
    <div className="app-container">
      {/* Header */}
      <header className="app-header">
        <h1>PodPico</h1>
        <div className="add-podcast-section">
          <input
            type="text"
            placeholder="Enter RSS feed URL..."
            value={rssUrl}
            onChange={e => setRssUrl(e.target.value)}
            disabled={addingPodcast}
            onKeyPress={e => e.key === 'Enter' && addPodcast()}
          />
          <button
            onClick={addPodcast}
            disabled={addingPodcast || !rssUrl.trim()}
          >
            {addingPodcast ? 'Adding...' : 'Add Podcast'}
          </button>
        </div>
        {error && <div className="error-message">{error}</div>}
      </header>

      {/* Main Content - 3-Pane Layout */}
      <main className="main-content">
        {/* Left Sidebar - Podcast Folders */}
        <aside className="sidebar">
          <h2>Podcasts</h2>

          {/* Combined Inbox - User Story #7 */}
          <div
            className={`podcast-item ${!selectedPodcast ? 'selected' : ''}`}
            onClick={() => setSelectedPodcast(null)}
          >
            <span className="podcast-icon">📥</span>
            <span className="podcast-name">Combined Inbox</span>
            {getTotalNewEpisodes() > 0 && (
              <span className="episode-count">({getTotalNewEpisodes()})</span>
            )}
          </div>

          {/* Individual Podcasts */}
          <div className="podcast-list">
            {podcasts.map(podcast => (
              <div
                key={podcast.id}
                className={`podcast-item ${selectedPodcast?.id === podcast.id ? 'selected' : ''}`}
                onClick={() => setSelectedPodcast(podcast)}
              >
                <span className="podcast-icon">🎙️</span>
                <span className="podcast-name">{podcast.name}</span>
                {podcast.new_episode_count > 0 && (
                  <span className="episode-count">
                    ({podcast.new_episode_count})
                  </span>
                )}
              </div>
            ))}
          </div>

          {podcasts.length === 0 && (
            <div className="empty-state">
              <p>No podcasts yet.</p>
              <p>Add your first podcast using the form above!</p>
            </div>
          )}
        </aside>

        {/* Middle Pane - Episode List */}
        <section className="episode-list-pane">
          <header className="pane-header">
            <h2>
              {selectedPodcast
                ? `${selectedPodcast.name} Episodes`
                : 'All New Episodes'}
            </h2>
            
            {/* User Story #12: Search for episodes within a podcast */}
            {selectedPodcast && (
              <div className="search-section">
                <div className="search-input-container">
                  <input
                    type="text"
                    placeholder="Search episodes..."
                    value={searchQuery}
                    onChange={e => setSearchQuery(e.target.value)}
                    className="search-input"
                  />
                  {searchQuery && (
                    <button
                      type="button"
                      onClick={() => setSearchQuery('')}
                      className="search-clear-button"
                      title="Clear search"
                      aria-label="Clear search"
                    >
                      ✕
                    </button>
                  )}
                </div>
                {isSearching && <span className="search-loading">⏳</span>}
                {isSearchMode && (
                  <span className="search-results-count">
                    {searchResults.length} result{searchResults.length !== 1 ? 's' : ''}
                  </span>
                )}
              </div>
            )}
            
            <div className="episode-count-info">
              {isSearchMode
                ? `${searchResults.length} result${searchResults.length !== 1 ? 's' : ''}`
                : `${episodes?.length || 0} episode${(episodes?.length || 0) !== 1 ? 's' : ''}`}
            </div>
          </header>

          {loading || isSearching ? (
            <div className="loading">
              {isSearching ? 'Searching episodes...' : 'Loading episodes...'}
            </div>
          ) : (
            <div className="episode-list">
              {/* User Story #12: Show search results or regular episodes */}
              {(isSearchMode ? searchResults : episodes)?.map(episode => (
                <div
                  key={episode.id}
                  className={`episode-item ${selectedEpisode?.id === episode.id ? 'selected' : ''}`}
                  onClick={() => setSelectedEpisode(episode)}
                >
                  {/* User Story #6: See episode status within each podcast */}
                  <div className="episode-status">
                    <span className="status-icon" title={episode.status}>
                      {getStatusIcon(episode.status)}
                    </span>
                  </div>

                  <div className="episode-info">
                    <h3 className="episode-title">
                      {isSearchMode ? highlightText(episode.title, searchQuery) : episode.title}
                    </h3>
                    <div className="episode-meta">
                      {!selectedPodcast && (
                        <span className="podcast-name">
                          {episode.podcast_name} •{' '}
                        </span>
                      )}
                      <span className="episode-date">
                        {formatDate(episode.published_date)}
                      </span>
                      {episode.duration && (
                        <span className="episode-duration">
                          {' '}
                          • {formatDuration(episode.duration)}
                        </span>
                      )}
                    </div>
                  </div>

                  {/* User Story #5: Mark episodes as listened */}
                  <div
                    className="episode-actions"
                    onClick={e => e.stopPropagation()}
                  >
                    <select
                      value={episode.status}
                      onChange={e =>
                        updateEpisodeStatus(episode.id, e.target.value)
                      }
                      className="status-selector"
                    >
                      <option value="new">New</option>
                      <option value="unlistened">Unlistened</option>
                      <option value="listened">Listened</option>
                    </select>
                  </div>
                </div>
              ))}

              {/* User Story #12: Handle empty search results */}
              {(isSearchMode ? searchResults.length === 0 : (!episodes || episodes.length === 0)) && !loading && !isSearching && (
                <div className="empty-state">
                  {isSearchMode
                    ? `No episodes found matching "${searchQuery}"`
                    : selectedPodcast
                    ? `No episodes found for ${selectedPodcast.name}`
                    : 'No new episodes across all podcasts'}
                </div>
              )}
            </div>
          )}
        </section>

        {/* Right Pane - Episode Details */}
        <section className="episode-details-pane">
          {selectedEpisode ? (
            <div className="episode-details">
              <header className="episode-header">
                <h2>
                  {/* User Story #12: Highlight search terms in episode details title */}
                  {isSearchMode && searchQuery ? highlightText(selectedEpisode.title, searchQuery) : selectedEpisode.title}
                </h2>
                <div className="episode-meta-detailed">
                  <div className="meta-row">
                    <span className="meta-label">Podcast:</span>
                    <span>{selectedEpisode.podcast_name}</span>
                  </div>
                  <div className="meta-row">
                    <span className="meta-label">Published:</span>
                    <span>{formatDate(selectedEpisode.published_date)}</span>
                  </div>
                  {selectedEpisode.duration && (
                    <div className="meta-row">
                      <span className="meta-label">Duration:</span>
                      <span>{formatDuration(selectedEpisode.duration)}</span>
                    </div>
                  )}
                  <div className="meta-row">
                    <span className="meta-label">Status:</span>
                    <span className="status-with-icon">
                      {getStatusIcon(selectedEpisode.status)}{' '}
                      {selectedEpisode.status}
                    </span>
                  </div>
                </div>
              </header>

              {selectedEpisode.description && (
                <div className="episode-description">
                  <h3>Description</h3>
                  <p>
                    {/* User Story #12: Highlight search terms in episode description */}
                    {isSearchMode && searchQuery ? highlightText(selectedEpisode.description, searchQuery) : selectedEpisode.description}
                  </p>
                </div>
              )}

              <div className="episode-actions-detailed">
                <div className="status-controls">
                  <label>Mark as:</label>
                  <div className="status-buttons">
                    <button
                      className={
                        selectedEpisode.status === 'new' ? 'active' : ''
                      }
                      onClick={() =>
                        updateEpisodeStatus(selectedEpisode.id, 'new')
                      }
                    >
                      🔴 New
                    </button>
                    <button
                      className={
                        selectedEpisode.status === 'unlistened' ? 'active' : ''
                      }
                      onClick={() =>
                        updateEpisodeStatus(selectedEpisode.id, 'unlistened')
                      }
                    >
                      🔵 Unlistened
                    </button>
                    <button
                      className={
                        selectedEpisode.status === 'listened' ? 'active' : ''
                      }
                      onClick={() =>
                        updateEpisodeStatus(selectedEpisode.id, 'listened')
                      }
                    >
                      ✅ Listened
                    </button>
                  </div>
                </div>

                <div className="future-actions">
                  <button disabled title="Coming in User Story #3">
                    📥 Download Episode
                  </button>
                  <button disabled title="Coming in User Story #9">
                    📱 Transfer to Device
                  </button>
                </div>
              </div>
            </div>
          ) : (
            <div className="no-selection">
              <h2>Select an Episode</h2>
              <p>
                Choose an episode from the list to view details and manage its
                status.
              </p>
            </div>
          )}
        </section>
      </main>
    </div>
  )
}

export default App
