import { useEffect, useState, type JSX } from 'react'
import './Results.css'
import { type ScoredUrls, type sitemetadata } from './models/ScoredUrl'
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

    let sortedURLs =  Object.entries(result.urls).sort((a, b) => b[1] - a[1]);

    // console.log(sortedURLs);
    
    sortedURLs.forEach(e => {

      //@ts-ignore
      let metadata: sitemetadata = result.metadata[e[0]]
      element.push(
        <div key={e[0]} className='search_result'>
          <label>{metadata.title}</label><br />
          <a href={e[0]} target='_blank'>{e[0]}</a>
        </div>
      )
      // element.push(
      element.push(<br key={e[0] + "br"}/>)
    })
    
    if(element.length == 0) {
      return [<p>No results found.</p>]
    }
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
