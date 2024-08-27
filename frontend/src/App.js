import React from 'react';
import { BrowserRouter as Router, Route, Link, Routes } from 'react-router-dom';
import PostSchedulerForm from './PostSchedulerForm';
import PostQuery from './PostQuery';
import UpdatePost from './UpdatePost';

const App = () => {
    return (
        <Router>
            <div className="container mt-5">
                <nav className="navbar navbar-expand-lg navbar-light bg-light">
                    <div className="navbar-nav">
                        <Link to="/" className="nav-item nav-link">Schedule Post</Link>
                        <Link to="/query" className="nav-item nav-link">Query Posts</Link>
                    </div>
                </nav>
                <Routes>
                    <Route path="/" element={<PostSchedulerForm />} />
                    <Route path="/query" element={<PostQuery />} />
                    <Route path="/update" element={<UpdatePost />} />
                </Routes>
            </div>
        </Router>
    );
};

export default App;