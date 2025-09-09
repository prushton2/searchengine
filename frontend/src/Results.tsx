import { useEffect, useState, type JSX } from 'react'
import './Results.css'
import { type ScoredUrls } from './models/ScoredUrl'
import { Search } from './API'

function Results() {
  const [searchResult, setSearchResult] = useState<ScoredUrls | undefined>(undefined)
  const [searchBoxContents, setSearchBoxContents] = useState<string>("")

    useEffect(() => {
      const params = new URLSearchParams(window.location.search);
      const query = params.get('q');
      
      if (query) {
        setSearchBoxContents(query);
        init(query)
      }

      async function init(query: string) {
        let scoredUrls = await Search(query)
        setSearchResult(scoredUrls);
      }

    }, [])

    function runSearch() {
      window.location.href=`/search?q=${searchBoxContents}`
    }
  
  function formatSearchResult(result: ScoredUrls): JSX.Element[] {
    let element: JSX.Element[] = []
    
    Object.entries(result.urls).forEach(e => {
      element.push(<div className='search_result'>
        <a href={e[0]}>{e[0]}</a>

      </div>)
      // element.push(
      element.push(<br />)
    })
    
    return element
  }

  return (
    <div className='App_body'>
      <div className="card">
        <input value={searchBoxContents} onChange={(e) => setSearchBoxContents(e.target.value) }/>
        <button onClick={() => runSearch()}>
          Search
        </button>
      </div>
      {searchResult == undefined ? "" : formatSearchResult(searchResult)}
    </div>
  )
}


export default Results
