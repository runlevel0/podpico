# PodPico

## Product Vision

PodPico is a desktop podcast management application designed for podcast enthusiasts who primarily listen to episodes on USB-compatible audio devices (such as MP3 players or non-smartphone mobile devices).

### Core Value Proposition
PodPico simplifies podcast management by providing a streamlined interface for:
- Managing podcast subscriptions and episode downloads
- Transferring episodes to USB-compatible audio devices
- Tracking episode status (new, unlistened, listened)
- Maintaining synchronization between PC and USB device

### Key Differentiators
- Focused on USB device integration rather than streaming
- Simple, lightweight interface without unnecessary features
- Clear episode status management
- Optimized for manual file transfer workflows

### Target Users
- Podcast listeners who prefer dedicated audio devices
- Users who want to manage their podcast library offline
- People who need clear organization of their podcast episodes
- Users who want to maintain a simple, focused podcast management workflow

### Problem Solved
PodPico addresses the challenge of managing podcast episodes across PC and USB devices by providing:
- Clear overview of episode status
- Simple transfer management
- Basic but essential podcast management features
- No unnecessary complexity or resource-heavy features

## Key Features and Functionality

### Core Features (MVP)
1. Podcast Management
   - Add/remove individual podcast subscriptions via RSS feed URLs
   - Organize podcasts in a hierarchical view (podcast → episodes)
   - View podcast episodes per podcast subscription with basic metadata (title, date, duration)
   - Download episodes to local storage on a per-podcast basis
   - Basic episode filtering and sorting within each podcast

2. Episode Status Management
   - Mark episodes as: New, Unlistened, Listened
   - Visual indicators for episode status
   - Status persistence across sessions

3. USB Device Integration
   - Detect connected USB devices
   - View available storage space
   - Transfer episodes to device
   - Remove episodes from device
   - Basic device storage management

4. User Interface
   - Simple, intuitive layout
   - Clear status indicators
   - Basic search functionality
   - Episode list view with status information

### Future Enhancements
1. Advanced Features
   - Batch operations (download/transfer multiple episodes)
   - Custom episode sorting
   - Episode metadata editing
   - Backup/restore functionality

2. Device Support
   - Support for additional device types
   - Automatic device detection
   - Device-specific optimizations

3. User Experience
   - Customizable interface
   - Keyboard shortcuts
   - Advanced filtering options
   - Episode notes/comments

### Technical Requirements
1. System Requirements
   - Desktop application (Windows/Linux)
   - Minimal system resources
   - Offline functionality
   - USB device compatibility

2. Data Management
   - Local database for podcast subscriptions
   - Episode metadata storage
   - Status tracking
   - Basic error handling and recovery

## User Stories

### Podcast Management
1. As a podcast listener, I want to add a new podcast subscription via RSS URL so that I can keep track of new episodes from that specific show.
2. As a podcast listener, I want to view all episodes of a specific podcast so that I can manage my listening queue for that show.
3. As a podcast listener, I want to download episodes from a specific podcast to my computer so that I can transfer them to my USB device later.
4. As a podcast listener, I want to remove podcast subscriptions I no longer listen to so that I can keep my library organized.

### Episode Status Management
5. As a podcast listener, I want to mark episodes as "listened" within a specific podcast so that I can track my progress through that show.
6. As a podcast listener, I want to see the status of all episodes (new/unlistened/listened) within each podcast so that I can easily identify what to listen to next in that series.
7. As a podcast listener, I want to view all new episodes across all my podcast subscriptions so that I can quickly find new content.

### USB Device Integration
8. As a podcast listener, I want to see my connected USB device's storage capacity so that I can manage my available space.
9. As a podcast listener, I want to transfer selected episodes from a specific podcast to my USB device so that I can listen to them on the go.
10. As a podcast listener, I want to remove specific episodes from my USB device so that I can free up space for new episodes.
11. As a podcast listener, I want to see which episodes from each podcast are currently on my USB device so that I can avoid duplicate transfers.

### User Interface
12. As a podcast listener, I want to search for specific episodes within a podcast so that I can quickly find what I'm looking for.
13. As a podcast listener, I want to see clear status indicators for episodes within each podcast so that I can easily understand their current state.
14. As a podcast listener, I want to sort episodes by date within a specific podcast so that I can listen to them in chronological order.
15. As a podcast listener, I want to filter episodes by their status (listened/unlistened/new) within a specific podcast.

### Future Enhancements
16. As a podcast listener, I want to transfer multiple episodes from various podcasts at once so that I can save time.
17. As a podcast listener, I want to batch update episode statuses within a podcast so I can efficiently manage my library.
18. As a podcast listener, I want to see aggregated statistics about my podcast listening habits by podcast.

## Success Metrics

### User-Centered Metrics
1. **Adoption Rate**
   - Number of active users
   - User retention rate (% of users who continue using the app after 1, 3, 6 months)
   - Number of podcast subscriptions per user

2. **Usability Metrics**
   - Time to complete common tasks (e.g., adding a podcast, transferring episodes)
   - Error rate during key operations
   - Number of support requests related to usability issues

### Technical Performance Metrics
1. **Application Performance**
   - Average application start time
   - Memory usage
   - CPU usage during common operations
   - Storage efficiency

2. **Reliability**
   - Crash frequency
   - Error rates for podcast updates and downloads
   - USB device detection success rate
   - Data loss incidents

3. **Compatibility**
   - Success rate across supported operating systems
   - Success rate across different USB device types
   - RSS feed compatibility coverage


## Release Prioritization
For the MVP (Minimum Viable Product), prioritize features in this order:
****
1. **Must Have (MVP)**
   - Basic podcast subscription management
   - Episode downloading and status tracking
   - USB device detection and file transfer
   - Simple, functional UI with episode list views

2. **Should Have (Post-MVP)**
   - Search and advanced filtering
   - Batch operations
   - Performance optimizations
   - Enhanced UI improvements

3. **Could Have (Future Releases)**
   - Additional device support
   - Advanced data management
   - User customization options
   - Statistics and reporting

## Key Screens and Views

### Main Application Layout (Email-App Inspired)

#### Three-Pane Layout Structure
```
┌─────────────────┬─────────────────┬─────────────────┐
│   Left Sidebar  │  Episode List   │ Episode Details │
│   (Podcasts)    │    (Middle)     │    (Right)      │
│                 │                 │                 │
│ ■ Combined      │ Episode 1       │ Episode Title   │
│   Inbox (3)     │ Episode 2       │ Description     │
│                 │ Episode 3       │ Duration        │
│ ■ Podcast A (2) │ ...             │ Download Button │
│ ■ Podcast B     │                 │ Transfer Button │
│ ■ Podcast C (1) │                 │ Status Controls │
│                 │                 │                 │
└─────────────────┴─────────────────┴─────────────────┘
```

### Core Interface Components

#### 1. Left Sidebar - Podcast Folders
- **Purpose**: Navigation and overview of all podcast subscriptions
- **Key Elements**:
  - **Combined Inbox** folder (shows all new episodes across podcasts)
  - Individual podcast "folders" with:
    - Podcast name and artwork (small icon)
    - Badge/number showing new episode count
    - Visual indicators for podcasts with new content
  - **Add Podcast** button at bottom
  - **USB Device** section showing connected device status
- **User Stories Addressed**: #1, #4, #7, #8

#### 2. Middle Pane - Episode List
- **Purpose**: Display episodes from selected podcast folder or combined inbox
- **Key Elements**:
  - Episode list with compact view showing:
    - Episode title and date
    - Duration and status icons
    - Download status indicators
    - USB device presence indicators
  - Column headers for sorting (Date, Title, Duration, Status)
  - Filter controls at top (All, New, Unlistened, Listened)
  - Search bar
  - Multi-select checkboxes for batch operations
- **User Stories Addressed**: #2, #3, #5, #6, #12, #13, #14, #15

#### 3. Right Pane - Episode Details
- **Purpose**: Detailed view and actions for selected episode
- **Key Elements**:
  - Large episode title and podcast name
  - Full episode description
  - Episode metadata (date, duration, file size)
  - Playback preview controls (optional for MVP)
  - **Download** button with progress indicator
  - **Transfer to USB** button
  - **Mark as Listened/Unlistened** buttons
  - Episode artwork (if available)
- **User Stories Addressed**: #3, #5, #9, #10

### Secondary Screens/Dialogs

#### 4. Add Podcast Dialog
- **Purpose**: Add new podcast subscription
- **Key Elements**:
  - RSS URL input field
  - Podcast preview (overlay or modal)
  - Subscribe confirmation
- **User Stories Addressed**: #1

#### 5. USB Device Management Panel
- **Purpose**: Dedicated view for USB device operations
- **Key Elements**:
  - Device info and storage visualization
  - List of episodes on device (organized by podcast)
  - Bulk transfer queue and progress
  - Remove episodes functionality
- **User Stories Addressed**: #8, #9, #10, #11
- **Access**: Via button in left sidebar or dedicated tab

#### 6. Transfer Progress Overlay
- **Purpose**: Show active file transfers
- **Key Elements**:
  - Progress bars for individual transfers
  - Queue management
  - Cancel/retry options
- **Behavior**: Appears as overlay or bottom panel during transfers

### Navigation and Interaction Flow

#### Primary Workflow:
1. **Select folder** (Combined Inbox or specific podcast) in left sidebar
2. **Browse episodes** in middle pane
3. **Select episode** to view details in right pane
4. **Take actions** (download, transfer, mark status) from right pane

#### Key Interactions:
- **Combined Inbox**: Shows all new episodes with podcast name visible
- **Podcast Folders**: Shows only episodes from that specific podcast
- **Badge Numbers**: Update in real-time as episodes are marked or new ones arrive
- **Multi-select**: Enable batch operations from middle pane
- **Right-click menus**: Quick actions on episodes and podcasts
- **Auto-mark as Read**: Episodes automatically change from "New" to "Unlistened" when their details are displayed in the right pane (similar to email behavior)

#### Episode Status Progression:
1. **New** → **Unlistened** (automatic when episode details viewed)
2. **Unlistened** → **Listened** (manual action via button or right-click)
3. **Listened** → **Unlistened** (manual action if user wants to re-listen)

### Design Considerations

1. **Email-App Familiarity**: Users immediately understand the folder structure and three-pane layout
2. **Efficient Scanning**: Combined inbox allows quick discovery of new content across all podcasts
3. **Focused Management**: Individual podcast folders enable detailed episode management
4. **Visual Hierarchy**: Clear separation between navigation, list, and details
5. **Responsive Splits**: Panes can be resized; layout can switch to horizontal split if preferred
6. **Status Visibility**: Consistent visual language for episode status throughout all panes
7. **Batch Operations**: Multi-select in middle pane enables efficient bulk actions
8. **Automatic Status Updates**: Episodes auto-mark as "read" when viewed, reducing manual actions and badge counts update immediately
9. **Visual Feedback**: Clear visual distinction between New/Unlistened/Listened states with immediate updates when status changes

## Context Menus (Right-Click)

### Left Sidebar - Podcast Folder Context Menu
**Activated on**: Individual podcast folders

**Menu Items**:
- **Refresh Feed** - Update episodes for this specific podcast
- **Mark All as Listened** - Mark all episodes in this podcast as listened
- **Mark All as Unlistened** - Reset all episodes to unlistened status
- **Download New Episodes** - Download all new episodes from this podcast
- **Transfer All New to USB** - Transfer all new episodes to connected USB device
- **Open Podcast Website** - Open the podcast's main website in browser
- **Podcast Properties** - View/edit podcast details (RSS URL, custom name, etc.)
- **Unsubscribe** - Remove this podcast subscription (with confirmation)

### Middle Pane - Episode List Context Menu
**Activated on**: Individual episodes in the list

**Single Episode Menu**:
- **Download Episode** - Download episode to local storage
- **Transfer to USB** - Copy episode to connected USB device
- **Remove from USB** - Delete episode from USB device (if present)
- **Mark as Listened** - Change status to listened
- **Mark as Unlistened** - Change status to unlistened
- **Mark as New** - Reset status to new
- **Copy Episode URL** - Copy direct download link to clipboard
- **Open in Browser** - Open episode page/description in web browser
- **Episode Properties** - View detailed episode information

**Multiple Episodes Selected Menu**:
- **Download Selected** - Download all selected episodes
- **Transfer Selected to USB** - Copy all selected episodes to USB device
- **Remove Selected from USB** - Delete all selected episodes from USB device
- **Mark Selected as Listened** - Bulk status change
- **Mark Selected as Unlistened** - Bulk status change
- **Mark Selected as New** - Bulk status change

### Middle Pane - Empty Area Context Menu
**Activated on**: Empty space in episode list

**Menu Items**:
- **Refresh This Podcast** - Update episodes for currently selected podcast
- **Select All Episodes** - Select all visible episodes in current view
- **Clear Selection** - Deselect all selected episodes

### USB Device Context Menu
**Activated on**: USB device section in left sidebar or episodes shown as "on device"

**Menu Items**:
- **Eject Device** - Safely remove USB device
- **View Device Contents** - Open detailed USB device management panel
- **Check Available Space** - Display storage information
- **Remove All Episodes** - Clear all podcast episodes from device (with confirmation)
- **Organize by Podcast** - Restructure device folders by podcast name

### Combined Inbox Context Menu
**Activated on**: Combined Inbox folder in left sidebar

**Menu Items**:
- **Refresh All Feeds** - Update episodes for all subscribed podcasts
- **Mark All as Listened** - Mark all new episodes as listened
- **Download All New** - Download all new episodes from all podcasts
- **Transfer All New to USB** - Transfer all new episodes to USB device

### Episode Details Pane Context Menu
**Activated on**: Episode description or metadata in right pane

**Menu Items**:
- **Copy Episode Title** - Copy episode title to clipboard
- **Copy Description** - Copy full episode description to clipboard
- **Copy Episode URL** - Copy direct download link
- **Select All Description** - Select all text in description for copying

## Acceptance Criteria for MVP User Stories

### Podcast Management

#### User Story #1: Add new podcast subscription via RSS URL
**Acceptance Criteria:**
- Given a valid RSS feed URL, when I paste it in the add podcast dialog, then the app validates the feed within 5 seconds
- Given a valid RSS feed, when I click subscribe, then the podcast appears in the left sidebar within 2 seconds
- Given an invalid URL, when I attempt to subscribe, then I receive a clear error message explaining the issue
- Given a valid feed with episodes, when I subscribe, then at least the 5 most recent episodes are fetched and marked as "New"
- Given I'm offline, when I attempt to add a podcast, then I receive an appropriate error message about network connectivity

#### User Story #2: View all episodes of a specific podcast
**Acceptance Criteria:**
- Given I select a podcast folder, when the episode list loads, then all episodes for that podcast are displayed within 3 seconds
- Given a podcast with episodes, when I view the episode list, then each episode shows title, date, duration, and status icon
- Given episodes exist, when I view them, then they are sorted by publication date (newest first) by default
- Given a podcast with many episodes, when I scroll through the list, then the interface remains responsive

#### User Story #3: Download episodes from a specific podcast
**Acceptance Criteria:**
- Given an episode not yet downloaded, when I click download, then a progress indicator appears immediately
- Given a download in progress, when I check the episode status, then I can see download progress percentage
- Given a successful download, when it completes, then the episode is marked as downloaded and available for transfer
- Given a failed download, when the error occurs, then I receive a retry option and clear error message
- Given insufficient disk space, when I attempt to download, then I receive a warning before the download starts

#### User Story #4: Remove podcast subscriptions
**Acceptance Criteria:**
- Given a subscribed podcast, when I right-click and select unsubscribe, then I receive a confirmation dialog
- Given I confirm unsubscription, when the action completes, then the podcast disappears from the sidebar immediately
- Given I unsubscribe from a podcast, when the action completes, then I'm offered the option to delete downloaded episodes
- Given episodes from an unsubscribed podcast are on USB, when I unsubscribe, then I'm notified about USB episodes

### Episode Status Management

#### User Story #5: Mark episodes as "listened"
**Acceptance Criteria:**
- Given an episode in any status, when I mark it as listened, then the status updates immediately in all views
- Given I mark an episode as listened, when viewing the podcast folder, then the new episode count decreases if applicable
- Given multiple episodes selected, when I bulk mark as listened, then all selected episodes update status simultaneously
- Given an episode on USB marked as listened, when I view USB contents, then the status is reflected there

#### User Story #6: See episode status within each podcast
**Acceptance Criteria:**
- Given episodes with different statuses, when I view a podcast, then each status is visually distinct (icons/colors)
- Given I filter by status, when I select "New" episodes, then only episodes with "New" status are shown
- Given episodes change status, when I refresh the view, then status indicators update immediately
- Given a large number of episodes, when I scan the list, then status indicators are clearly visible and consistent

#### User Story #7: View all new episodes across podcasts (Combined Inbox)
**Acceptance Criteria:**
- Given multiple podcasts with new episodes, when I select Combined Inbox, then all new episodes are shown with podcast names
- Given I view an episode in Combined Inbox, when the details load, then the episode automatically changes from "New" to "Unlistened"
- Given the Combined Inbox has episodes, when new episodes are fetched, then the view updates automatically
- Given I take actions on episodes in Combined Inbox, when I switch to individual podcast folders, then changes are reflected

### USB Device Integration

#### User Story #8: See USB device storage capacity
**Acceptance Criteria:**
- Given a USB device is connected, when detected, then available and used storage space is displayed within 5 seconds
- Given storage information is shown, when the display updates, then it shows both numerical (MB/GB) and visual (progress bar) indicators
- Given storage space changes, when files are added/removed, then the display updates within 2 seconds
- Given no USB device is connected, when I check the USB section, then I see a clear "No device connected" message

#### User Story #9: Transfer episodes to USB device
**Acceptance Criteria:**
- Given a downloaded episode and connected USB device, when I transfer an episode, then a progress indicator shows immediately
- Given a transfer in progress, when I check status, then I can see transfer speed and time remaining
- Given a successful transfer, when it completes, then the episode shows "on device" indicator
- Given a transfer fails, when the error occurs, then I receive a clear error message and retry option
- Given insufficient USB space, when I attempt transfer, then I'm warned before the transfer starts

#### User Story #10: Remove episodes from USB device
**Acceptance Criteria:**
- Given episodes on USB device, when I select remove, then I receive confirmation before deletion
- Given I confirm removal, when the action completes, then the episode no longer shows "on device" indicator
- Given I remove an episode from USB, when I check device storage, then available space increases accordingly
- Given multiple episodes selected for removal, when I confirm, then all selected episodes are removed together

#### User Story #11: See which episodes are on USB device
**Acceptance Criteria:**
- Given episodes transferred to USB, when I view any episode list, then "on device" indicators are clearly visible
- Given I'm viewing USB device contents, when the list loads, then episodes are organized by podcast
- Given episodes on device, when I compare with local episode list, then the on-device status is consistent across views
- Given device contents change, when I refresh views, then on-device indicators update within 3 seconds

### User Interface

#### User Story #12: Search for episodes within a podcast
**Acceptance Criteria:**
- Given I enter search terms, when I search within a podcast, then results appear within 2 seconds
- Given search results, when displayed, then matching text is highlighted in episode titles/descriptions
- Given I clear search, when the action completes, then the full episode list returns immediately
- Given no search results, when the search completes, then I see a clear "no results found" message

#### User Story #13: See clear status indicators
**Acceptance Criteria:**
- Given episodes with different statuses, when I view any list, then each status has a distinct visual indicator
- Given status indicators, when I see them, then they are consistent across all views (colors, icons, text)
- Given I'm colorblind, when I view status indicators, then status is clear through more than just color (icons, text)
- Given status changes, when it updates, then the visual change is immediate and noticeable

