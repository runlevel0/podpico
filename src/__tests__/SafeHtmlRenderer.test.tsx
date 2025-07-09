import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import SafeHtmlRenderer from '../SafeHtmlRenderer'

describe('SafeHtmlRenderer Component', () => {
  describe('HTML Rendering', () => {
    it('renders plain text content correctly', () => {
      render(<SafeHtmlRenderer htmlContent="This is plain text content" />)

      expect(screen.getByText('This is plain text content')).toBeInTheDocument()
    })

    it('renders HTML paragraphs correctly', () => {
      const htmlContent = '<p>First paragraph</p><p>Second paragraph</p>'
      render(<SafeHtmlRenderer htmlContent={htmlContent} />)

      expect(screen.getByText('First paragraph')).toBeInTheDocument()
      expect(screen.getByText('Second paragraph')).toBeInTheDocument()
    })

    it('renders HTML links correctly', () => {
      const htmlContent =
        '<p>Visit <a href="https://example.com">our website</a> for more info.</p>'
      render(<SafeHtmlRenderer htmlContent={htmlContent} />)

      const link = screen.getByRole('link', { name: 'our website' })
      expect(link).toBeInTheDocument()
      expect(link).toHaveAttribute('href', 'https://example.com')
    })

    it('renders emphasis and strong tags correctly', () => {
      const htmlContent =
        '<p>This is <em>emphasized</em> and this is <strong>strong</strong> text.</p>'
      render(<SafeHtmlRenderer htmlContent={htmlContent} />)

      expect(screen.getByText('emphasized')).toBeInTheDocument()
      expect(screen.getByText('strong')).toBeInTheDocument()
    })

    it('renders line breaks correctly', () => {
      const htmlContent = 'Line one<br/>Line two<br>Line three'
      const { container } = render(
        <SafeHtmlRenderer htmlContent={htmlContent} />
      )

      // Check that br tags are present in the rendered HTML
      expect(container.innerHTML).toContain('<br>')
      expect(screen.getByText(/Line one/)).toBeInTheDocument()
      expect(screen.getByText(/Line two/)).toBeInTheDocument()
      expect(screen.getByText(/Line three/)).toBeInTheDocument()
    })

    it('renders lists correctly', () => {
      const htmlContent = '<ul><li>First item</li><li>Second item</li></ul>'
      render(<SafeHtmlRenderer htmlContent={htmlContent} />)

      expect(screen.getByText('First item')).toBeInTheDocument()
      expect(screen.getByText('Second item')).toBeInTheDocument()
    })
  })

  describe('Security and Sanitization', () => {
    it('removes dangerous script tags', () => {
      const maliciousContent =
        '<p>Safe content</p><script>alert("xss")</script>'
      const { container } = render(
        <SafeHtmlRenderer htmlContent={maliciousContent} />
      )

      expect(screen.getByText('Safe content')).toBeInTheDocument()
      expect(container.innerHTML).not.toContain('<script>')
      expect(container.innerHTML).not.toContain('alert')
    })

    it('removes dangerous event handlers', () => {
      const maliciousContent = '<p onclick="alert(\'xss\')">Click me</p>'
      const { container } = render(
        <SafeHtmlRenderer htmlContent={maliciousContent} />
      )

      expect(screen.getByText('Click me')).toBeInTheDocument()
      expect(container.innerHTML).not.toContain('onclick')
      expect(container.innerHTML).not.toContain('alert')
    })

    it('removes dangerous iframe tags', () => {
      const maliciousContent =
        '<p>Safe content</p><iframe src="javascript:alert(1)"></iframe>'
      const { container } = render(
        <SafeHtmlRenderer htmlContent={maliciousContent} />
      )

      expect(screen.getByText('Safe content')).toBeInTheDocument()
      expect(container.innerHTML).not.toContain('<iframe>')
    })

    it('preserves safe attributes on links', () => {
      const htmlContent =
        '<a href="https://example.com" target="_blank" rel="noopener">Safe link</a>'
      render(<SafeHtmlRenderer htmlContent={htmlContent} />)

      const link = screen.getByRole('link', { name: 'Safe link' })
      expect(link).toHaveAttribute('href', 'https://example.com')
      expect(link).toHaveAttribute('target', '_blank')
      expect(link).toHaveAttribute('rel', 'noopener')
    })
  })

  describe('Search Highlighting', () => {
    it('highlights search terms in plain text', () => {
      const content = 'This is a test episode about React development'
      render(<SafeHtmlRenderer htmlContent={content} searchQuery="React" />)

      const highlightedText = screen.getByText('React')
      expect(highlightedText).toBeInTheDocument()
      expect(highlightedText.tagName.toLowerCase()).toBe('mark')
      expect(highlightedText).toHaveClass('search-highlight')
    })

    it('highlights search terms in HTML content', () => {
      const htmlContent =
        '<p>This episode covers <em>React</em> testing fundamentals.</p>'
      render(<SafeHtmlRenderer htmlContent={htmlContent} searchQuery="React" />)

      const highlightedText = screen.getByText('React')
      expect(highlightedText).toBeInTheDocument()
      expect(highlightedText.tagName.toLowerCase()).toBe('mark')
      expect(highlightedText).toHaveClass('search-highlight')
    })

    it('highlights multiple occurrences of search terms', () => {
      const content =
        'React is great. Learn React development with React tutorials.'
      render(<SafeHtmlRenderer htmlContent={content} searchQuery="React" />)

      const highlightedElements = screen.getAllByText('React')
      expect(highlightedElements).toHaveLength(3)
      highlightedElements.forEach(element => {
        expect(element.tagName.toLowerCase()).toBe('mark')
        expect(element).toHaveClass('search-highlight')
      })
    })

    it('performs case-insensitive search highlighting', () => {
      const content = 'React and react are the same framework'
      render(<SafeHtmlRenderer htmlContent={content} searchQuery="REACT" />)

      const highlightedElements = screen.getAllByText(/react/i)
      expect(highlightedElements).toHaveLength(2)
      highlightedElements.forEach(element => {
        expect(element.tagName.toLowerCase()).toBe('mark')
      })
    })

    it('does not highlight when search query is empty', () => {
      const content = 'This content should not be highlighted'
      const { container } = render(
        <SafeHtmlRenderer htmlContent={content} searchQuery="" />
      )

      expect(screen.getByText(content)).toBeInTheDocument()
      expect(container.innerHTML).not.toContain('<mark>')
    })

    it('escapes special regex characters in search query', () => {
      const content = 'Episode about $variables and (parameters)'
      render(
        <SafeHtmlRenderer htmlContent={content} searchQuery="$variables" />
      )

      const highlightedText = screen.getByText('$variables')
      expect(highlightedText).toBeInTheDocument()
      expect(highlightedText.tagName.toLowerCase()).toBe('mark')
    })
  })

  describe('Real-world Podcast Description Examples', () => {
    it('handles complex podcast description with paragraphs and emphasis', () => {
      const podcastDescription =
        '<p>In this episode, Pastor Jon continues our 1 Peter sermon series, <em>Citizens</em>, by examining the apostle Peter\'s exhortations to the church in the midst of suffering. Pastor Jon in turn offers his exhortation to our church today in the midst of secularism. Also included in this episode is one of the first worship songs written by our community, <em>We Behold</em>. // 2019-08-25</p> <p> </p> <p>"But rejoice inasmuch as you participate in the sufferings of Christ, so that you may be overjoyed when his glory is revealed."</p> <p>1 Peter 4: 13</p> <p> </p> <p>Church of the City New York<br /> https://church.nyc | @COTCNYC</p>'

      render(<SafeHtmlRenderer htmlContent={podcastDescription} />)

      expect(screen.getByText('Citizens')).toBeInTheDocument()
      expect(screen.getByText('We Behold')).toBeInTheDocument()
      expect(screen.getByText('1 Peter 4: 13')).toBeInTheDocument()
      expect(
        screen.getByText(/Church of the City New York/)
      ).toBeInTheDocument()
    })

    it('handles podcast description with links and line breaks', () => {
      const podcastDescription =
        'Find <a href="https://open.spotify.com/show/2naS9Y5nWSli9vi9bOvGxp">all the episodes from this season here</a>. And <a href="https://www.npr.org/series/1015448333/planet-money-summer-school">past seasons here</a>. <br/><br/>Planet Money Summer School has arrived at the birth of the United States and the chance to set up a whole new economy from scratch.'

      render(<SafeHtmlRenderer htmlContent={podcastDescription} />)

      const spotifyLink = screen.getByRole('link', {
        name: 'all the episodes from this season here',
      })
      expect(spotifyLink).toHaveAttribute(
        'href',
        'https://open.spotify.com/show/2naS9Y5nWSli9vi9bOvGxp'
      )

      const nprLink = screen.getByRole('link', { name: 'past seasons here' })
      expect(nprLink).toHaveAttribute(
        'href',
        'https://www.npr.org/series/1015448333/planet-money-summer-school'
      )

      expect(screen.getByText(/Planet Money Summer School/)).toBeInTheDocument()
    })

    it('handles search highlighting in complex podcast descriptions', () => {
      const podcastDescription =
        '<p>This episode covers <em>React</em> testing fundamentals and best practices for testing React components.</p>'

      render(
        <SafeHtmlRenderer
          htmlContent={podcastDescription}
          searchQuery="React"
        />
      )

      const highlightedElements = screen.getAllByText('React')
      expect(highlightedElements).toHaveLength(2)
      highlightedElements.forEach(element => {
        expect(element.tagName.toLowerCase()).toBe('mark')
        expect(element).toHaveClass('search-highlight')
      })
    })
  })

  describe('CSS Classes and Styling', () => {
    it('applies custom className to the container', () => {
      const { container } = render(
        <SafeHtmlRenderer
          htmlContent="<p>Test content</p>"
          className="custom-class"
        />
      )

      expect(container.firstChild).toHaveClass('custom-class')
    })

    it('applies default empty className when none provided', () => {
      const { container } = render(
        <SafeHtmlRenderer htmlContent="<p>Test content</p>" />
      )

      // Check that the className attribute is empty or has no specific classes
      expect(container.firstChild).toHaveAttribute('class', '')
    })
  })
})
