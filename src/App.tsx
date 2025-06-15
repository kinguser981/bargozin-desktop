import "./index.css";
import { Routes, Route } from "react-router";
import About from "./pages/about";
import DomainTest from "./pages/domain-test";
import Layout from "./components/layout";
import DownloadTest from "./pages/download";
import DockerTest from "./pages/docker";

function App() {
  return (
    <div>
      <Layout>
        <Routes>
          <Route path="/" element={<DomainTest />} />
          <Route path="/about" element={<About />} />
          <Route path="/download" element={<DownloadTest />} />
          <Route path="/docker" element={<DockerTest />} />
        </Routes>
      </Layout>
    </div>
  );
}

export default App;
