import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useProblems, ProblemWithMastery } from '../hooks/useProblems';
import './ProblemBrowser.css';

const ProblemBrowser: React.FC = () => {
  const navigate = useNavigate();
  const { listProblems, deleteProblem } = useProblems();

  const [problems, setProblems] = useState<ProblemWithMastery[]>([]);
  const [loading, setLoading] = useState(true);
  const [filter, setFilter] = useState<string>('all');
  const [sort, setSort] = useState<string>('created');

  useEffect(() => {
    loadProblems();
  }, [filter, sort]);

  const loadProblems = async () => {
    try {
      setLoading(true);
      const response = await listProblems(filter, sort, 50, 0);
      setProblems(response.problems);
    } catch (error) {
      console.error('Failed to load problems:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this problem?')) {
      try {
        await deleteProblem(id);
        loadProblems();
      } catch (error) {
        console.error('Failed to delete problem:', error);
        alert('Failed to delete problem');
      }
    }
  };

  const getDifficultyLabel = (difficulty: number) => {
    const labels = ['', 'Easy', 'Medium-Easy', 'Medium', 'Medium-Hard', 'Hard'];
    return labels[difficulty] || 'Unknown';
  };

  if (loading) {
    return (
      <div className="page-container">
        <h1>Problem Browser</h1>
        <p>Loading problems...</p>
      </div>
    );
  }

  return (
    <div className="page-container">
      <div className="browser-header">
        <h1>Problem Browser</h1>
        <button onClick={() => navigate('/problems/new')} className="btn-primary">
          + New Problem
        </button>
      </div>

      <div className="filter-controls">
        <div className="filter-group">
          <label>Filter:</label>
          <select value={filter} onChange={(e) => setFilter(e.target.value)}>
            <option value="all">All</option>
            <option value="solved">Solved</option>
            <option value="unsolved">Unsolved</option>
          </select>
        </div>

        <div className="filter-group">
          <label>Sort by:</label>
          <select value={sort} onChange={(e) => setSort(e.target.value)}>
            <option value="created">Newest First</option>
            <option value="difficulty">Difficulty</option>
            <option value="mastery">Mastery %</option>
          </select>
        </div>
      </div>

      <div className="problems-grid">
        {problems.length === 0 ? (
          <p className="empty-state">No problems found. Create your first problem!</p>
        ) : (
          problems.map(({ problem, mastery, tags }) => (
            <div key={problem.id} className="problem-card">
              <div className="problem-header">
                <h3 onClick={() => navigate(`/problems/${problem.id}`)} className="problem-title">
                  {problem.title}
                </h3>
                <span className={`difficulty-badge difficulty-${problem.difficulty}`}>
                  {getDifficultyLabel(problem.difficulty)}
                </span>
              </div>

              <p className="problem-description">
                {problem.description || 'No description'}
              </p>

              <div className="problem-tags">
                {tags.map((tag) => (
                  <span key={tag} className="tag">{tag}</span>
                ))}
              </div>

              <div className="problem-stats">
                <span>Mastery: {mastery?.mastery_percent.toFixed(0)}%</span>
                <span>{mastery?.solved ? '✓ Solved' : '○ Unsolved'}</span>
                <span>Attempts: {mastery?.attempt_count || 0}</span>
              </div>

              <div className="problem-actions">
                <button onClick={() => navigate(`/problems/${problem.id}`)} className="btn-secondary">
                  View
                </button>
                <button onClick={() => handleDelete(problem.id)} className="btn-danger">
                  Delete
                </button>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default ProblemBrowser;
