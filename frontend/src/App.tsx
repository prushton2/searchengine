import { useState } from 'react'
import './App.css'

function App() {
  const [searchBoxContents, setSearchBoxContents] = useState<string>("")

  function runSearch() {
    window.location.href=`/search?q=${searchBoxContents}`
  }

  return (
    <>
      <h1>Search</h1>
      <div className="card">
        <input onChange={(e) => setSearchBoxContents(e.target.value) }/>
        <button onClick={() => runSearch()}>
          Search
        </button>
      </div>
    </>
  )
}


export default App
