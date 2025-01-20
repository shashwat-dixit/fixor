import { useState, useEffect } from 'react'

function App() {
  const [message, setMessage] = useState('')

  useEffect(() => {
    fetch('http://localhost:3001/api/hello')
      .then(response => response.json())
      .then(data => setMessage(data.text))
  }, [])

  return (
    <div>
      <h1>Rust + React with Turborepo</h1>
      <p>Message from backend: {message}</p>
    </div>
  )
}

export default App
