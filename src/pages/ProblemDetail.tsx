import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useProblems, ProblemWithMastery } from '../hooks/useProblems';
import './ProblemDetail.css';

const ProblemDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { getProblem, updateProblem, addTag } = useProblems();

  const [problem, setProblem] = useState<ProblemWithMastery | null>(null);
  const [loading, setLoading] = useState(true);
  const [editing, setEditing] = useState(false);
  const [formData, setFormData] = useState({
    title: '',
    description: '',
    difficulty: 3,
  });
  const [newTag, setNewTag] = useState('');

  useEffect(() => {
    if (id) {
      loadProblem();
    }
  }, [id]);

  const loadProblem = async () => {
    try {
      setLoading(true);
      const data = await getProblem(id!);
      setProblem(data);
      setFormData({
        title: data.problem.title,
        description: data.problem.description || '',
        difficulty: data.problem.difficulty,
      });
    } catch (error) {
      console.error('Failed to load problem:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    try {
      await updateProblem(id!, formData.title, formData.description, formData.difficulty);
      setEditing(false);
      loadProblem();
    } catch (error) {
      console.error('Failed to update problem:', error);
      alert('Failed to update problem');
    }
  };

  const handleAddTag = async () => {
    if (newTag.trim()) {
      try {
        await addTag(id!, newTag.trim());
        setNewTag('');
        loadProblem();
      } catch (error) {
        console.error('Failed to add tag:', error);
      }
    }
  };

  if (loading || !problem) {
    return (
      <div className="page-container">
        <p>Loading...</p>
      </div>
    );
  }

  return (
    <div className="page-container">
      <div className="detail-header">
        <button onClick={() => navigate('/problems')} className="btn-back">
          ← Back to Problems
        </button>
        {!editing && (
          <button onClick={() => setEditing(true)} className="btn-primary">
            Edit
          </button>
        )}
      </div>

      {editing ? (
        <div className="edit-form">
          <div className="form-group">
            <label>Title:</label>
            <input
              type="text"
              value={formData.title}
              onChange={(e) => setFormData({ ...formData, title: e.target.value })}
            />
          </div>

          <div className="form-group">
            <label>Description:</label>
            <textarea
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              rows={5}
            />
          </div>

          <div className="form-group">
            <label>Difficulty (1-5):</label>
            <input
              type="number"
              min="1"
              max="5"
              value={formData.difficulty}
              onChange={(e) => setFormData({ ...formData, difficulty: parseInt(e.target.value) })}
            />
          </div>

          <div className="form-actions">
            <button onClick={handleSave} className="btn-primary">
              Save
            </button>
            <button onClick={() => setEditing(false)} className="btn-secondary">
              Cancel
            </button>
          </div>
        </div>
      ) : (
        <div className="problem-detail">
          <h1>{problem.problem.title}</h1>
          <p className="description">{problem.problem.description || 'No description'}</p>

          <div className="metadata">
            <div className="meta-item">
              <strong>Difficulty:</strong> {problem.problem.difficulty}/5
            </div>
            <div className="meta-item">
              <strong>Mastery:</strong> {problem.mastery?.mastery_percent.toFixed(0)}%
            </div>
            <div className="meta-item">
              <strong>Status:</strong> {problem.mastery?.solved ? 'Solved' : 'Unsolved'}
            </div>
            <div className="meta-item">
              <strong>Attempts:</strong> {problem.mastery?.attempt_count || 0}
            </div>
          </div>

          <div className="tags-section">
            <h3>Tags</h3>
            <div className="tags-list">
              {problem.tags.map((tag) => (
                <span key={tag} className="tag">{tag}</span>
              ))}
            </div>
            <div className="add-tag">
              <input
                type="text"
                placeholder="Add tag..."
                value={newTag}
                onChange={(e) => setNewTag(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && handleAddTag()}
              />
              <button onClick={handleAddTag} className="btn-primary">Add</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default ProblemDetail;
