import React from 'react';
import { useParams } from 'react-router-dom';

const ProblemDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  
  return (
    <div className="page-container">
      <h1>Problem Detail</h1>
      <p>Problem ID: {id}</p>
      <p>View, edit, and mastery progress will appear here.</p>
    </div>
  );
};

export default ProblemDetail;
