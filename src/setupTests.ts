import '@testing-library/jest-dom'

// Mock Tauri API for testing
const mockInvoke = vi.fn()

// Mock the Tauri invoke function
vi.mock('@tauri-apps/api/core', () => ({
  invoke: mockInvoke,
}))

// Export mock for tests to use
export { mockInvoke }

// Mock podcast data for testing
export const MOCK_PODCAST = {
  id: 1,
  name: 'Test Podcast',
  rss_url: 'https://example.com/feed.xml',
  description: 'A test podcast',
  artwork_url: 'https://example.com/artwork.jpg',
  website_url: 'https://example.com',
  last_updated: '2024-01-01T00:00:00Z',
  episode_count: 5,
  new_episode_count: 2,
}

export const MOCK_EPISODE = {
  id: 1,
  podcast_id: 1,
  podcast_name: 'Test Podcast',
  title: 'Test Episode',
  description: 'A test episode',
  episode_url: 'https://example.com/episode.mp3',
  published_date: '2024-01-01T00:00:00Z',
  duration: 3600,
  file_size: 50000000,
  local_file_path: null,
  status: 'new',
  downloaded: false,
  on_device: false,
}

// Reset mocks before each test
beforeEach(() => {
  mockInvoke.mockReset()
})
