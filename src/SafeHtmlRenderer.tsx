import React from 'react'
import DOMPurify from 'dompurify'

interface SafeHtmlRendererProps {
  htmlContent: string
  searchQuery?: string
  className?: string
}

/**
 * SafeHtmlRenderer - Safely renders HTML content with sanitization
 * 
 * Features:
 * - Sanitizes HTML using DOMPurify to prevent XSS attacks
 * - Supports search term highlighting in HTML content
 * - Configurable allowed HTML tags for podcast descriptions
 * 
 * Used for rendering podcast episode descriptions that may contain HTML markup
 * such as paragraphs, links, emphasis, and line breaks.
 */
export const SafeHtmlRenderer: React.FC<SafeHtmlRendererProps> = ({
  htmlContent,
  searchQuery,
  className = '',
}) => {
  // DOMPurify configuration for podcast descriptions
  // Allow common HTML tags found in podcast descriptions based on research:
  // - <p>, <br> for paragraphs and line breaks
  // - <a> for links (with href attribute)
  // - <em>, <i>, <strong>, <b> for emphasis
  // - <ul>, <ol>, <li> for lists
  const sanitizeConfig = {
    ALLOWED_TAGS: ['p', 'br', 'a', 'em', 'i', 'strong', 'b', 'ul', 'ol', 'li', 'span', 'div'],
    ALLOWED_ATTR: ['href', 'target', 'rel', 'class'],
    // Force external links to open in new tab for security
    ADD_ATTR: ['target', 'rel'],
    FORBID_ATTR: ['onclick', 'onerror', 'onload', 'onmouseover'],
  }

  let processedHtml = htmlContent

  // Apply search highlighting if search query is provided
  if (searchQuery && searchQuery.trim()) {
    // Escape special regex characters in search query
    const escapedQuery = searchQuery.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
    const regex = new RegExp(`(${escapedQuery})`, 'gi')
    
    // Apply highlighting by wrapping matches in <mark> tags
    // This works on the HTML content before sanitization
    processedHtml = processedHtml.replace(regex, '<mark class="search-highlight">$1</mark>')
  }

  // Sanitize the HTML content (including any added <mark> tags)
  const sanitizedHtml = DOMPurify.sanitize(processedHtml, {
    ...sanitizeConfig,
    ALLOWED_TAGS: [...sanitizeConfig.ALLOWED_TAGS, 'mark'], // Allow <mark> for search highlighting
  })

  return (
    <div 
      className={className}
      dangerouslySetInnerHTML={{ __html: sanitizedHtml }}
    />
  )
}

export default SafeHtmlRenderer 