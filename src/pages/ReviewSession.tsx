import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useFsrs, DueCard } from '../hooks/useFsrs';
import './ReviewSession.css';

const RATING_LABELS = [
  '', // 0 (unused)
  'Forgot',       // 1
  'Struggling',   // 2
  'Hard',         // 3
  'Hard (OK)',    // 4
  'Moderate',     // 5
  'Moderate+',    // 6
  'Easy',         // 7
  'Very Easy',    // 8
  'Trivial',      // 9
  'Instant',      // 10
];

const ReviewSession: React.FC = () => {
  const navigate = useNavigate();
  const { getNextDue, processReview } = useFsrs();

  const [dueCards, setDueCards] = useState<DueCard[]>([]);
  const [currentIndex, setCurrentIndex] = useState(0);
  const [loading, setLoading] = useState(true);
  const [reviewing, setReviewing] = useState(false);
  const [startTime, setStartTime] = useState(Date.now());

  useEffect(() => {
    loadDueCards();
  }, []);

  const loadDueCards = async () => {
    try {
      setLoading(true);
      const cards = await getNextDue(20);
      setDueCards(cards);
      setStartTime(Date.now());
    } catch (error) {
      console.error('Failed to load due cards:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleRating = async (rating: number) => {
    if (reviewing || dueCards.length === 0) return;

    try {
      setReviewing(true);
      const currentCard = dueCards[currentIndex];
      const elapsedSeconds = Math.floor((Date.now() - startTime) / 1000);

      await processReview(currentCard.card_id, rating, elapsedSeconds);

      // Move to next card
      if (currentIndex + 1 < dueCards.length) {
        setCurrentIndex(currentIndex + 1);
        setStartTime(Date.now());
      } else {
        // Finished all cards
        alert('🎉 All reviews completed!');
        navigate('/dashboard');
      }
    } catch (error) {
      console.error('Failed to process review:', error);
      alert('Failed to submit review');
    } finally {
      setReviewing(false);
    }
  };

  if (loading) {
    return (
      <div className="page-container">
        <h1>Loading reviews...</h1>
      </div>
    );
  }

  if (dueCards.length === 0) {
    return (
      <div className="page-container">
        <h1>No Reviews Due</h1>
        <p>Come back later!</p>
        <button onClick={() => navigate('/dashboard')} className="btn-primary">
          Back to Dashboard
        </button>
      </div>
    );
  }

  const currentCard = dueCards[currentIndex];

  return (
    <div className="review-container">
      <div className="review-header">
        <div className="progress-info">
          <span>Card {currentIndex + 1} of {dueCards.length}</span>
          <span>State: {currentCard.state}</span>
          <span>Overdue: {currentCard.days_overdue} days</span>
        </div>
      </div>

      <div className="problem-card-large">
        <h2>{currentCard.title}</h2>
        <div className="card-metadata">
          <span className={`difficulty-badge difficulty-${currentCard.difficulty}`}>
            Difficulty: {currentCard.difficulty}/5
          </span>
          <span>Stability: {currentCard.stability.toFixed(1)} days</span>
          <span>Reviews: {currentCard.reps}</span>
          <span>Lapses: {currentCard.lapses}</span>
        </div>
      </div>

      <div className="rating-section">
        <h3>How well did you know this?</h3>
        <div className="rating-grid">
          {[1, 2, 3, 4, 5, 6, 7, 8, 9, 10].map((rating) => (
            <button
              key={rating}
              className={`rating-button rating-${rating}`}
              onClick={() => handleRating(rating)}
              disabled={reviewing}
            >
              <span className="rating-number">{rating}</span>
              <span className="rating-label">{RATING_LABELS[rating]}</span>
            </button>
          ))}
        </div>

        <div className="rating-legend">
          <div className="legend-item">
            <span className="legend-color lapse"></span>
            <span>1-2: Lapse (forgot)</span>
          </div>
          <div className="legend-item">
            <span className="legend-color hard"></span>
            <span>3-4: Hard</span>
          </div>
          <div className="legend-item">
            <span className="legend-color good"></span>
            <span>5-6: Good</span>
          </div>
          <div className="legend-item">
            <span className="legend-color easy"></span>
            <span>7-10: Easy</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ReviewSession;
