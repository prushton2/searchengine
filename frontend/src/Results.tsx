import { useEffect, useState, type JSX } from 'react'
import './Results.css'
import { type ScoredUrls } from './models/ScoredUrl'
import { Search } from './API'

function Results() {
  const [searchResult, setSearchResult] = useState<ScoredUrls | undefined>(undefined)
  const [searchBoxContents, setSearchBoxContents] = useState<string>("")

    useEffect(() => {
      async function init() {
        let scoredUrls = await Search("searchBoxContents")
      }
      init()
    }, [])

    function runSearch() {
      window.location.href=`/search?q=${searchBoxContents}`
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
      <div className="card">
        <input onChange={(e) => setSearchBoxContents(e.target.value) }/>
        <button onClick={() => runSearch()}>
          Search
        </button>
        {searchResult == undefined ? "" : formatSearchResult(searchResult)}
      </div>
    </>
  )
}


export default Results
