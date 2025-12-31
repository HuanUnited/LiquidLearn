import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Navigation from './components/Navigation';
import Dashboard from './pages/Dashboard';
import ReviewSession from './pages/ReviewSession';
import ProblemBrowser from './pages/ProblemBrowser';
import ProblemDetail from './pages/ProblemDetail';
import Analytics from './pages/Analytics';
import './App.css';

const App: React.FC = () => {
  return (
    <Router>
      <div className="app">
        <Navigation />
        <main className="main-content">
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/problems" element={<ProblemBrowser />} />
            <Route path="/problems/:id" element={<ProblemDetail />} />
            <Route path="/review" element={<ReviewSession />} />
            <Route path="/analytics" element={<Analytics />} />
          </Routes>
        </main>
      </div>
    </Router>
  );
};

export default App;
