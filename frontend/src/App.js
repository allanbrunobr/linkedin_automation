import React from 'react';
import { BrowserRouter as Router, Route, Link, Routes } from 'react-router-dom';
import PostSchedulerForm from './PostSchedulerForm';
import PostQuery from './PostQuery';
import UpdatePost from './UpdatePost';
import './styles.css';

const App = () => {
    return (
        <Router>
            <div className="app-container">
                <div className="form-container">
                    <nav className="navbar navbar-expand-lg navbar-light bg-light">
                        <div className="navbar-nav">
                            <Link to="/" className="nav-item nav-link">Schedule Post</Link>
                            <Link to="/query" className="nav-item nav-link">Query Posts</Link>
                        </div>
                    </nav>
                </div>
                <div className="form-container">
                    <Routes>
                        <Route path="/" element={<PostSchedulerForm />} />
                        <Route path="/query" element={<PostQuery />} />
                        <Route path="/update" element={<UpdatePost />} />
                    </Routes>
                </div>
            </div>
        </Router>
    );
};

export default App;