import React, { useState, useEffect, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/core'
import './App.css'
import SafeHtmlRenderer from './SafeHtmlRenderer'

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

// User Story #3: Download Episodes - Progress tracking interface
interface DownloadProgress {
  episode_id: number
  downloaded_bytes: number
  total_bytes: number
  percentage: number
  speed_bps: number
  eta_seconds: number
}

// User Story #8: USB Device Management - Interface for USB devices
interface UsbDevice {
  id: string
  name: string
  path: string
  total_space: number
  available_space: number
  is_connected: boolean
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

  // User Story #3: Download Episodes - State management
  const [downloadingEpisodes, setDownloadingEpisodes] = useState<Set<number>>(
    new Set()
  )
  const [downloadProgress, setDownloadProgress] = useState<
    Map<number, DownloadProgress>
  >(new Map())
  const [downloadErrors, setDownloadErrors] = useState<Map<number, string>>(
    new Map()
  )

  // Remove Downloaded Episodes - State management
  const [removingEpisodes, setRemovingEpisodes] = useState<Set<number>>(
    new Set()
  )
  const [removeEpisodeErrors, setRemoveEpisodeErrors] = useState<
    Map<number, string>
  >(new Map())

  // User Story #4: Remove Podcasts - State management
  const [removingPodcasts, setRemovingPodcasts] = useState<Set<number>>(
    new Set()
  )
  const [removeErrors, setRemoveErrors] = useState<Map<number, string>>(
    new Map()
  )
  const [showRemoveConfirm, setShowRemoveConfirm] = useState<number | null>(
    null
  )

  // User Story #8: USB Device Management - State management
  const [usbDevices, setUsbDevices] = useState<UsbDevice[]>([])
  const [usbLoading, setUsbLoading] = useState(false)
  const [usbError, setUsbError] = useState<string | null>(null)

  // Load podcasts on component mount
  useEffect(() => {
    loadPodcasts()
  }, [])

  // User Story #8: Load USB devices on component mount
  useEffect(() => {
    loadUsbDevices()
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

  // User Story #8: Load USB devices from backend
  async function loadUsbDevices() {
    setUsbLoading(true)
    setUsbError(null)

    try {
      // User Story #8 Acceptance Criteria: Detection within 5 seconds
      const startTime = Date.now()

      const devices: UsbDevice[] = await invoke('get_usb_devices')

      const detectionTime = Date.now() - startTime
      if (detectionTime > 5000) {
        // eslint-disable-next-line no-console
        console.warn(
          `User Story #8: USB detection took ${detectionTime}ms, should be under 5 seconds`
        )
      }

      setUsbDevices(devices)
    } catch (err) {
      // eslint-disable-next-line no-console
      console.error('Failed to load USB devices:', err)
      setUsbError(`Failed to detect USB devices: ${err}`)
      setUsbDevices([]) // Clear devices on error
    } finally {
      setUsbLoading(false)
    }
  }

  async function addPodcast() {
    console.log('DEBUG: Starting addPodcast function')

    if (!rssUrl.trim()) {
      console.log('DEBUG: RSS URL is empty')
      setError('Please enter an RSS URL')
      return
    }

    console.log('DEBUG: Adding podcast with URL:', rssUrl)
    setAddingPodcast(true)
    setError('')

    try {
      // User Story #1: Add new podcast subscription via RSS URL
      console.log('DEBUG: Calling invoke add_podcast')
      await invoke('add_podcast', { rssUrl })
      console.log('DEBUG: Successfully added podcast')

      console.log('DEBUG: Reloading podcasts')
      await loadPodcasts()
      console.log('DEBUG: Podcasts reloaded')

      setRssUrl('')
      setError('')
      console.log('DEBUG: addPodcast completed successfully')
    } catch (err) {
      // eslint-disable-next-line no-console
      console.error('DEBUG: Error in addPodcast:', err)
      setError(`Failed to add podcast: ${err}`)
    } finally {
      setAddingPodcast(false)
      console.log('DEBUG: addPodcast function ended')
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
  const searchEpisodes = useCallback(
    async (query: string) => {
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
    },
    [selectedPodcast]
  )

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
    if (!podcasts) return 0
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

    const regex = new RegExp(
      `(${searchQuery.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`,
      'gi'
    )
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

  // User Story #3: Download Episodes - Core functionality
  // Acceptance Criteria: Progress tracking, file existence validation, error messages
  async function downloadEpisode(episode: Episode) {
    console.log(
      'DEBUG: Starting downloadEpisode for episode:',
      episode.id,
      episode.title
    )

    if (downloadingEpisodes.has(episode.id)) {
      console.log('DEBUG: Episode already downloading, returning')
      return // Already downloading
    }

    console.log('DEBUG: Clearing previous errors for episode:', episode.id)
    // Clear any previous error for this episode
    setDownloadErrors(prev => {
      const newErrors = new Map(prev)
      newErrors.delete(episode.id)
      return newErrors
    })

    console.log('DEBUG: Marking episode as downloading')
    // Mark episode as downloading
    setDownloadingEpisodes(prev => new Set(prev).add(episode.id))

    try {
      console.log(
        'DEBUG: About to call invoke download_episode with episodeId:',
        episode.id
      )
      // Start download
      await invoke('download_episode', {
        episodeId: episode.id,
      })
      console.log('DEBUG: invoke download_episode completed successfully')

      console.log('DEBUG: Starting progress tracking')
      // Start progress tracking
      startProgressTracking(episode.id)
    } catch (err) {
      // eslint-disable-next-line no-console
      console.error('DEBUG: Failed to start download:', err)
      setDownloadErrors(prev =>
        new Map(prev).set(episode.id, `Download failed: ${err}`)
      )
      setDownloadingEpisodes(prev => {
        const newSet = new Set(prev)
        newSet.delete(episode.id)
        return newSet
      })
    }
  }

  // User Story #3: Progress tracking for download episodes
  // Acceptance Criteria: Progress indicators with percentage display
  function startProgressTracking(episodeId: number) {
    console.log('DEBUG: Starting progress tracking for episode:', episodeId)

    // Start checking progress immediately (not after 1 second delay)
    let progressInterval: number

    const checkProgress = async () => {
      try {
        console.log('DEBUG: Requesting progress for episode:', episodeId)
        const progress: DownloadProgress = await invoke(
          'get_download_progress',
          {
            episodeId: episodeId,
          }
        )

        console.log('DEBUG: Received progress data:', progress)

        // Validate progress data before setting state
        if (progress && typeof progress.percentage === 'number') {
          console.log(
            'DEBUG: Setting progress state for episode:',
            episodeId,
            'progress:',
            progress.percentage
          )
          setDownloadProgress(prev => new Map(prev).set(episodeId, progress))

          // Check if download is complete (100% or episode is now downloaded)
          if (progress.percentage >= 100) {
            console.log('DEBUG: Download completed for episode:', episodeId)
            clearInterval(progressInterval)

            // Remove from downloading set
            setDownloadingEpisodes(prev => {
              const newSet = new Set(prev)
              newSet.delete(episodeId)
              return newSet
            })

            // Clear progress tracking
            setDownloadProgress(prev => {
              const newMap = new Map(prev)
              newMap.delete(episodeId)
              return newMap
            })

            // Only refresh episodes data, don't change the selected episode/podcast
            // This prevents the view from changing focus
            console.log(
              'DEBUG: Refreshing episodes data without changing selection'
            )
            if (selectedPodcast) {
              await loadEpisodes(selectedPodcast.id)
            } else {
              await loadEpisodes(null)
            }

            // Only refresh podcasts for episode count updates
            await loadPodcasts()
          }
        } else {
          // Invalid or no progress data - this is normal for very fast downloads
          console.log(
            'DEBUG: No progress data available yet for episode:',
            episodeId
          )

          // For fast downloads, check if the episode is already downloaded
          // This can happen when the download completes before progress tracking starts
          if (selectedPodcast) {
            await loadEpisodes(selectedPodcast.id)
          } else {
            await loadEpisodes(null)
          }

          // Check if this episode is now marked as downloaded
          const episodes = (await invoke('get_episodes', {
            podcastId: selectedPodcast?.id || null,
          })) as Episode[]

          const updatedEpisode = episodes.find(e => e.id === episodeId)
          if (updatedEpisode?.downloaded) {
            console.log(
              'DEBUG: Episode completed during fast download, stopping progress tracking'
            )
            clearInterval(progressInterval)

            // Remove from downloading set
            setDownloadingEpisodes(prev => {
              const newSet = new Set(prev)
              newSet.delete(episodeId)
              return newSet
            })

            // Clear progress tracking
            setDownloadProgress(prev => {
              const newMap = new Map(prev)
              newMap.delete(episodeId)
              return newMap
            })

            // Refresh podcasts for episode count updates
            await loadPodcasts()
            return
          }
        }
      } catch (err) {
        // eslint-disable-next-line no-console
        console.error('DEBUG: Progress tracking error:', err)
        clearInterval(progressInterval)

        setDownloadErrors(prev =>
          new Map(prev).set(episodeId, `Progress tracking failed: ${err}`)
        )

        setDownloadingEpisodes(prev => {
          const newSet = new Set(prev)
          newSet.delete(episodeId)
          return newSet
        })

        setDownloadProgress(prev => {
          const newMap = new Map(prev)
          newMap.delete(episodeId)
          return newMap
        })
      }
    }

    // Check progress immediately
    checkProgress()

    // Then set up interval to check every 500ms (faster than before for better responsiveness)
    progressInterval = setInterval(checkProgress, 500)
  }

  // User Story #3: Format file size for display
  function formatFileSize(bytes: number): string {
    if (bytes === 0) return '0 Bytes'

    const k = 1024
    const sizes = ['Bytes', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))

    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
  }

  // User Story #3: Format download speed for display
  function formatSpeed(bytesPerSecond: number): string {
    return formatFileSize(bytesPerSecond) + '/s'
  }

  // User Story #3: Format time remaining for display
  function formatTimeRemaining(seconds: number): string {
    if (seconds < 60) {
      return `${Math.round(seconds)}s`
    } else if (seconds < 3600) {
      const minutes = Math.floor(seconds / 60)
      const remainingSeconds = seconds % 60
      if (remainingSeconds > 0) {
        return `${minutes}m ${remainingSeconds}s`
      } else {
        return `${minutes}m`
      }
    } else {
      const hours = Math.floor(seconds / 3600)
      const remainingMinutes = Math.floor((seconds % 3600) / 60)
      if (remainingMinutes > 0) {
        return `${hours}h ${remainingMinutes}m`
      } else {
        return `${hours}h`
      }
    }
  }

  // Remove Downloaded Episode - Core functionality
  async function removeDownloadedEpisode(episode: Episode) {
    if (removingEpisodes.has(episode.id)) {
      return // Already removing
    }

    // Clear any previous error for this episode
    setRemoveEpisodeErrors(prev => {
      const newErrors = new Map(prev)
      newErrors.delete(episode.id)
      return newErrors
    })

    // Mark episode as being removed
    setRemovingEpisodes(prev => new Set(prev).add(episode.id))

    try {
      // Call backend to remove downloaded episode
      await invoke('delete_downloaded_episode', {
        episodeId: episode.id,
      })

      // Refresh podcasts and episodes to update UI
      await loadPodcasts()
      if (selectedPodcast) {
        await loadEpisodes(selectedPodcast.id)
      } else {
        await loadEpisodes(null)
      }
    } catch (err) {
      // eslint-disable-next-line no-console
      console.error('Failed to remove downloaded episode:', err)
      setRemoveEpisodeErrors(prev =>
        new Map(prev).set(episode.id, `Remove failed: ${err}`)
      )
    } finally {
      // Remove from removing set
      setRemovingEpisodes(prev => {
        const newSet = new Set(prev)
        newSet.delete(episode.id)
        return newSet
      })
    }
  }

  // User Story #8: USB Device Storage Formatting Functions
  function formatStorageSize(bytes: number | undefined | null): string {
    // Handle undefined, null, or non-finite values
    if (bytes == null || !isFinite(bytes) || bytes < 0) {
      return '0 B'
    }

    if (bytes === 0) return '0 B'

    const units = ['B', 'KB', 'MB', 'GB', 'TB']
    const k = 1024
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    const size = bytes / Math.pow(k, i)

    // Format to 1 decimal place for readability
    return `${size.toFixed(1)} ${units[i]}`
  }

  function calculateStorageUsagePercentage(
    device: UsbDevice | undefined | null
  ): number {
    // Handle undefined/null device or invalid properties
    if (
      !device ||
      device.total_space == null ||
      device.available_space == null ||
      !isFinite(device.total_space) ||
      !isFinite(device.available_space) ||
      device.total_space <= 0
    ) {
      return 0
    }

    const usedSpace = device.total_space - device.available_space
    const percentage = (usedSpace / device.total_space) * 100

    // Ensure we return a valid, finite number
    return isFinite(percentage) ? Math.round(percentage) : 0
  }

  // User Story #4: Remove Podcasts - Core functionality
  // Acceptance Criteria: Confirmation dialog, cleanup options, immediate UI updates
  async function removePodcast(podcastId: number) {
    if (removingPodcasts.has(podcastId)) {
      return // Already removing
    }

    // Clear any previous error for this podcast
    setRemoveErrors(prev => {
      const newErrors = new Map(prev)
      newErrors.delete(podcastId)
      return newErrors
    })

    // Mark podcast as being removed
    setRemovingPodcasts(prev => new Set(prev).add(podcastId))

    try {
      // Call backend to remove podcast
      await invoke('remove_podcast', {
        podcastId,
      })

      // Hide confirmation dialog
      setShowRemoveConfirm(null)

      // Refresh podcasts to update UI
      await loadPodcasts()

      // Clear selection if this podcast was selected
      if (selectedPodcast?.id === podcastId) {
        setSelectedPodcast(null)
        setSelectedEpisode(null)
        setEpisodes([])
      }

      // Refresh combined inbox if that's selected
      if (!selectedPodcast) {
        await loadEpisodes(null)
      }
    } catch (err) {
      // eslint-disable-next-line no-console
      console.error('Failed to remove podcast:', err)
      setRemoveErrors(prev =>
        new Map(prev).set(podcastId, `Remove failed: ${err}`)
      )
    } finally {
      // Remove from removing set
      setRemovingPodcasts(prev => {
        const newSet = new Set(prev)
        newSet.delete(podcastId)
        return newSet
      })
    }
  }

  // User Story #4: Handle remove confirmation
  function handleRemoveClick(podcastId: number, event: React.MouseEvent) {
    event.stopPropagation() // Prevent podcast selection
    setShowRemoveConfirm(podcastId)
  }

  // User Story #4: Cancel remove operation
  function cancelRemove() {
    setShowRemoveConfirm(null)
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
            {podcasts?.map(podcast => (
              <div
                key={podcast.id}
                className={`podcast-item ${selectedPodcast?.id === podcast.id ? 'selected' : ''}`}
                onClick={() => setSelectedPodcast(podcast)}
              >
                <div className="podcast-main">
                  <span className="podcast-icon">🎙️</span>
                  <span className="podcast-name">{podcast.name}</span>
                  {podcast.new_episode_count > 0 && (
                    <span className="episode-count">
                      ({podcast.new_episode_count})
                    </span>
                  )}
                </div>

                {/* User Story #4: Remove podcast button */}
                <div
                  className="podcast-actions"
                  onClick={e => e.stopPropagation()}
                >
                  {removingPodcasts.has(podcast.id) ? (
                    <span
                      className="removing-indicator"
                      title="Removing podcast..."
                    >
                      ⏳
                    </span>
                  ) : (
                    <button
                      className="remove-podcast-button"
                      onClick={e => handleRemoveClick(podcast.id, e)}
                      title={`Remove ${podcast.name}`}
                      aria-label={`Remove ${podcast.name}`}
                    >
                      🗑️
                    </button>
                  )}
                </div>

                {/* User Story #4: Show remove errors */}
                {removeErrors.has(podcast.id) && (
                  <div
                    className="podcast-error"
                    onClick={e => e.stopPropagation()}
                  >
                    <span className="error-message">
                      {removeErrors.get(podcast.id)}
                    </span>
                    <button
                      className="retry-button"
                      onClick={() => {
                        setRemoveErrors(prev => {
                          const newErrors = new Map(prev)
                          newErrors.delete(podcast.id)
                          return newErrors
                        })
                      }}
                      title="Dismiss error"
                    >
                      ✕
                    </button>
                  </div>
                )}
              </div>
            ))}
          </div>

          {(podcasts?.length || 0) === 0 && (
            <div className="empty-state">
              <p>No podcasts yet.</p>
              <p>Don&apos;t see your podcast? Add it above!</p>
            </div>
          )}

          {/* User Story #8: USB Device Section */}
          <div className="usb-device-section">
            <h3>USB Device</h3>

            {usbLoading ? (
              <div className="usb-loading">
                <span>🔍 Detecting devices...</span>
              </div>
            ) : usbError ? (
              <div className="usb-error">
                <span>⚠️ {usbError}</span>
                <button
                  className="retry-button"
                  onClick={loadUsbDevices}
                  data-testid="usb-retry-button"
                  title="Retry USB detection"
                >
                  🔄 Retry
                </button>
              </div>
            ) : !usbDevices || usbDevices.length === 0 ? (
              <div className="usb-no-device">
                <span>📱 No device connected</span>
              </div>
            ) : (
              <div className="usb-device-list">
                {usbDevices
                  .filter(device => device && device.id) // Filter out invalid devices
                  .map(device => (
                    <div key={device.id} className="usb-device-item">
                      <div className="device-info">
                        <div className="device-header">
                          <span className="device-icon">📱</span>
                          <span className="device-name">
                            {device.name || 'USB Device'}
                          </span>
                        </div>
                        <div className="storage-info">
                          <div className="storage-text">
                            {formatStorageSize(device.available_space)}{' '}
                            available of {formatStorageSize(device.total_space)}
                          </div>
                          <div
                            className="storage-progress-bar"
                            role="progressbar"
                            aria-label={`USB device storage: ${calculateStorageUsagePercentage(device)}% used`}
                            aria-valuenow={calculateStorageUsagePercentage(
                              device
                            )}
                            aria-valuemin={0}
                            aria-valuemax={100}
                          >
                            <div
                              className="storage-progress-fill"
                              style={{
                                width: `${calculateStorageUsagePercentage(device)}%`,
                              }}
                            />
                          </div>
                          <div className="storage-percentage">
                            {calculateStorageUsagePercentage(device)}% used
                          </div>
                        </div>
                      </div>
                    </div>
                  ))}
              </div>
            )}
          </div>
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
                    {searchResults?.length || 0} result
                    {(searchResults?.length || 0) !== 1 ? 's' : ''}
                  </span>
                )}
              </div>
            )}

            <div className="episode-count-info">
              {isSearchMode
                ? `${searchResults?.length || 0} result${(searchResults?.length || 0) !== 1 ? 's' : ''}`
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
              {Array.isArray(isSearchMode ? searchResults : episodes) &&
                (isSearchMode ? searchResults : episodes).map(episode => (
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
                        {isSearchMode
                          ? highlightText(episode.title, searchQuery)
                          : episode.title}
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

                        {/* User Story #3: Download status indicators in episode list */}
                        {episode.downloaded && (
                          <span className="download-indicator downloaded">
                            • 📥 Downloaded
                          </span>
                        )}
                        {downloadingEpisodes.has(episode.id) && (
                          <span className="download-indicator downloading">
                            • ⏳ Downloading
                            {downloadProgress.has(episode.id) && (
                              <span className="inline-progress">
                                (
                                {downloadProgress
                                  .get(episode.id)!
                                  .percentage.toFixed(0)}
                                %)
                              </span>
                            )}
                          </span>
                        )}
                        {downloadErrors.has(episode.id) && (
                          <span className="download-indicator error">
                            • ⚠️ Download Failed
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
              {(isSearchMode
                ? !Array.isArray(searchResults) || searchResults.length === 0
                : !Array.isArray(episodes) || episodes.length === 0) &&
                !loading &&
                !isSearching && (
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
                  {isSearchMode && searchQuery
                    ? highlightText(selectedEpisode.title, searchQuery)
                    : selectedEpisode.title}
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
                  {/* Render HTML content safely with search highlighting */}
                  <SafeHtmlRenderer
                    htmlContent={selectedEpisode.description}
                    searchQuery={isSearchMode ? searchQuery : undefined}
                    className="episode-description-content"
                  />
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

                <div className="download-actions">
                  {/* User Story #3: Download Episodes - Functional implementation */}
                  {selectedEpisode.downloaded ? (
                    <div className="download-status">
                      <span className="download-complete">✅ Downloaded</span>
                      {selectedEpisode.local_file_path && (
                        <small className="file-path">
                          {selectedEpisode.local_file_path}
                        </small>
                      )}

                      {/* Remove Downloaded Episode - Button */}
                      <div className="remove-download-actions">
                        {removingEpisodes.has(selectedEpisode.id) ? (
                          <span className="removing-indicator">
                            🗑️ Removing...
                          </span>
                        ) : (
                          <button
                            className="remove-download-button"
                            onClick={() =>
                              removeDownloadedEpisode(selectedEpisode)
                            }
                            title="Remove downloaded episode file"
                          >
                            🗑️ Remove Download
                          </button>
                        )}
                      </div>
                    </div>
                  ) : downloadingEpisodes.has(selectedEpisode.id) ? (
                    <div className="download-progress-container">
                      <div className="download-progress-header">
                        <span>📥 Downloading...</span>
                        {downloadProgress.has(selectedEpisode.id) && (
                          <span className="progress-percentage">
                            {downloadProgress
                              .get(selectedEpisode.id)!
                              .percentage.toFixed(1)}
                            %
                          </span>
                        )}
                      </div>

                      {downloadProgress.has(selectedEpisode.id) && (
                        <div className="download-progress-details">
                          <div className="progress-bar">
                            <div
                              className="progress-fill"
                              style={{
                                width: `${downloadProgress.get(selectedEpisode.id)!.percentage}%`,
                              }}
                            />
                          </div>

                          <div className="progress-info">
                            <span className="download-size">
                              {formatFileSize(
                                downloadProgress.get(selectedEpisode.id)!
                                  .downloaded_bytes
                              )}{' '}
                              /{' '}
                              {formatFileSize(
                                downloadProgress.get(selectedEpisode.id)!
                                  .total_bytes
                              )}
                            </span>
                            <span className="download-speed">
                              {formatSpeed(
                                downloadProgress.get(selectedEpisode.id)!
                                  .speed_bps
                              )}
                            </span>
                            <span className="download-eta">
                              ETA:{' '}
                              {formatTimeRemaining(
                                downloadProgress.get(selectedEpisode.id)!
                                  .eta_seconds
                              )}
                            </span>
                          </div>
                        </div>
                      )}
                    </div>
                  ) : (
                    <button
                      className="download-button"
                      onClick={() => downloadEpisode(selectedEpisode)}
                      title="Download this episode to your computer"
                    >
                      📥 Download Episode
                    </button>
                  )}

                  {/* User Story #3: Download error handling */}
                  {downloadErrors.has(selectedEpisode.id) && (
                    <div className="download-error">
                      <span className="error-icon">⚠️</span>
                      <span className="error-message">
                        {downloadErrors.get(selectedEpisode.id)}
                      </span>
                      <button
                        className="retry-button"
                        onClick={() => downloadEpisode(selectedEpisode)}
                        title="Retry download"
                        data-testid="download-retry-button"
                      >
                        🔄 Retry
                      </button>
                    </div>
                  )}

                  {/* Remove Downloaded Episode - Error handling */}
                  {removeEpisodeErrors.has(selectedEpisode.id) && (
                    <div className="remove-error">
                      <span className="error-icon">⚠️</span>
                      <span className="error-message">
                        {removeEpisodeErrors.get(selectedEpisode.id)}
                      </span>
                      <button
                        className="retry-button"
                        onClick={() => removeDownloadedEpisode(selectedEpisode)}
                        title="Retry removal"
                        data-testid="remove-retry-button"
                      >
                        🔄 Retry
                      </button>
                    </div>
                  )}
                </div>

                <div className="future-actions">
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

      {/* User Story #4: Remove Podcast Confirmation Dialog */}
      {showRemoveConfirm && (
        <div className="remove-confirm-overlay">
          <div
            className="remove-confirm-dialog"
            role="dialog"
            aria-labelledby="remove-confirm-title"
          >
            <div className="remove-confirm-header">
              <h3 id="remove-confirm-title">Remove Podcast</h3>
            </div>

            <div className="remove-confirm-content">
              <p>
                Are you sure you want to remove{' '}
                <strong>
                  {podcasts?.find(p => p.id === showRemoveConfirm)?.name}
                </strong>
                ?
              </p>

              <div className="remove-options">
                <p className="remove-warning">
                  ⚠️ This will permanently remove the podcast and all its
                  episodes from your library.
                </p>

                {/* Check if podcast has downloaded episodes */}
                {episodes?.some(
                  ep => ep.podcast_id === showRemoveConfirm && ep.downloaded
                ) && (
                  <div className="cleanup-warning">
                    <p>
                      📁 This podcast has downloaded episodes. They will be
                      removed from your computer.
                    </p>
                  </div>
                )}

                {/* Check if podcast has episodes on USB device */}
                {episodes?.some(
                  ep => ep.podcast_id === showRemoveConfirm && ep.on_device
                ) && (
                  <div className="device-warning">
                    <p>
                      📱 Some episodes from this podcast are on your USB device.
                      They will remain there but won't be managed by PodPico
                      anymore.
                    </p>
                  </div>
                )}
              </div>
            </div>

            <div className="remove-confirm-actions">
              <button
                className="cancel-button"
                onClick={cancelRemove}
                disabled={
                  showRemoveConfirm
                    ? removingPodcasts.has(showRemoveConfirm)
                    : false
                }
              >
                Cancel
              </button>
              <button
                className="remove-button"
                onClick={() =>
                  showRemoveConfirm && removePodcast(showRemoveConfirm)
                }
                disabled={
                  showRemoveConfirm
                    ? removingPodcasts.has(showRemoveConfirm)
                    : false
                }
              >
                {showRemoveConfirm &&
                removingPodcasts.has(showRemoveConfirm) ? (
                  <>⏳ Removing...</>
                ) : (
                  <>🗑️ Remove Podcast</>
                )}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

export default App
