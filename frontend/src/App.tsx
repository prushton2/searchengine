import { useState, type JSX } from 'react'
import './App.css'
import { type ScoredUrls } from './models/ScoredUrl'
import { Search } from './API'

function App() {
  const [searchBoxContents, setSearchBoxContents] = useState<string>("")
  const [searchResult, setSearchResult] = useState<ScoredUrls>()

  async function runSearch() {
    let scoredUrls = await Search(searchBoxContents)
    setSearchResult(scoredUrls)
    console.log("scoredUrls: ")
    console.log(scoredUrls)
  }
  
  function formatSearchResult(result: ScoredUrls): JSX.Element[] {
    let element: JSX.Element[] = []
    
    Object.entries(result.urls).forEach(e => {
      element.push(<a href={e[0]}>{e[0]}</a>)
      element.push(<br />)
    })
    
    return element
  }

  return (
    <>
      <h1>Search</h1>
      <div className="card">
        <input onChange={(e) => setSearchBoxContents(e.target.value) }/>
        <button onClick={() => runSearch()}>
          Search
        </button>
        <br />
        <br />
        {searchResult == undefined ? "" : formatSearchResult(searchResult)}
      </div>
    </>
  )
}


export default App
