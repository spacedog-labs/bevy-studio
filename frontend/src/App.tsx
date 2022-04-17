function App() {
  const test = () => {
    fetch("/api/echo");
  }
  return (
    <div className="App">
      <button onClick={test}>wow</button>
    </div>
  );
}

export default App;
