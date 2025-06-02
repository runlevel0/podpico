import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Podcast {
  id: number;
  name: string;
  rss_url: string;
  description?: string;
  artwork_url?: string;
  website_url?: string;
  last_updated?: string;
  episode_count: number;
  new_episode_count: number;
}

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [rssUrl, setRssUrl] = useState("");
  const [podcasts, setPodcasts] = useState<Podcast[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  async function addPodcast() {
    if (!rssUrl.trim()) {
      setError("Please enter an RSS URL");
      return;
    }

    setLoading(true);
    setError("");
    
    try {
      // User Story #1: Add new podcast subscription via RSS URL
      const podcast: Podcast = await invoke("add_podcast", { rssUrl });
      console.log("Added podcast:", podcast);
      
      // Refresh podcast list
      await loadPodcasts();
      setRssUrl("");
      setError("");
    } catch (err) {
      console.error("Failed to add podcast:", err);
      setError(`Failed to add podcast: ${err}`);
    } finally {
      setLoading(false);
    }
  }

  async function loadPodcasts() {
    try {
      const podcastList: Podcast[] = await invoke("get_podcasts");
      setPodcasts(podcastList);
    } catch (err) {
      console.error("Failed to load podcasts:", err);
      setError(`Failed to load podcasts: ${err}`);
    }
  }

  // Load podcasts on component mount
  useEffect(() => {
    loadPodcasts();
  }, []);

  return (
    <main className="container">
      <h1>PodPico - Podcast Manager</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <div className="section">
        <h2>Test Connection</h2>
        <form
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
            value={name}
          />
          <button type="submit">Greet</button>
        </form>
        <p>{greetMsg}</p>
      </div>

      <div className="section">
        <h2>Add Podcast (User Story #1)</h2>
        <form
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
            addPodcast();
          }}
        >
          <input
            id="rss-input"
            onChange={(e) => setRssUrl(e.currentTarget.value)}
            placeholder="Enter RSS feed URL..."
            value={rssUrl}
            disabled={loading}
          />
          <button type="submit" disabled={loading}>
            {loading ? "Adding..." : "Add Podcast"}
          </button>
        </form>
        {error && <p style={{ color: "red" }}>{error}</p>}
      </div>

      <div className="section">
        <h2>Your Podcasts ({podcasts.length})</h2>
        <button onClick={loadPodcasts}>Refresh</button>
        {podcasts.length === 0 ? (
          <p>No podcasts added yet. Try adding one above!</p>
        ) : (
          <div className="podcast-list">
            {podcasts.map((podcast) => (
              <div key={podcast.id} className="podcast-item">
                <h3>{podcast.name}</h3>
                <p><strong>URL:</strong> {podcast.rss_url}</p>
                {podcast.description && <p><strong>Description:</strong> {podcast.description}</p>}
                <p><strong>Episodes:</strong> {podcast.episode_count} total, {podcast.new_episode_count} new</p>
                {podcast.last_updated && <p><strong>Last Updated:</strong> {podcast.last_updated}</p>}
              </div>
            ))}
          </div>
        )}
      </div>

      <div className="section">
        <h3>Test RSS URLs</h3>
        <p>Try these popular podcast RSS feeds:</p>
        <ul>
          <li><button onClick={() => setRssUrl("https://feeds.npr.org/510289/podcast.xml")}>NPR Up First</button></li>
          <li><button onClick={() => setRssUrl("https://rss.cnn.com/rss/cnn_topstories.rss")}>CNN Top Stories</button></li>
          <li><button onClick={() => setRssUrl("https://feeds.megaphone.fm/HSW2398973788")}>Stuff You Should Know</button></li>
        </ul>
      </div>
    </main>
  );
}

export default App;
