import './index.css'
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { Routes, Route } from 'react-router-dom'
import App from './App.tsx'
import { BrowserRouter } from 'react-router-dom'
import Results from './Results.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<App />} />
        <Route path="/search" element={<Results />} />
      </Routes>
    </BrowserRouter>
  </StrictMode>,
)
