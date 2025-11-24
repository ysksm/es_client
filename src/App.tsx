import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { Layout } from './components/layout';
import { Dashboard, Connections, Indices, Extract, Database } from './pages';

function App() {
  return (
    <BrowserRouter>
      <Layout>
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/connections" element={<Connections />} />
          <Route path="/indices" element={<Indices />} />
          <Route path="/extract" element={<Extract />} />
          <Route path="/database" element={<Database />} />
        </Routes>
      </Layout>
    </BrowserRouter>
  );
}

export default App;
