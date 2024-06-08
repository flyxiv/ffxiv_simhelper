import QuickSimUIButton from "./components/quicksim";

const logo = process.env.PUBLIC_URL + "/images/logo.svg";


function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <QuickSimUIButton />
      </header>
    </div>
  );
}

export default App;
