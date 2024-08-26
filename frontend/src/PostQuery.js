import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import './PostQuery.css';
import './SharedStyles.css';

const PostQuery = () => {
    const [startDate, setStartDate] = useState('');
    const [endDate, setEndDate] = useState('');
    const [isSingleDay, setIsSingleDay] = useState(false);
    const [posts, setPosts] = useState([]);
    // eslint-disable-next-line no-unused-vars
    const [selectedPost, setSelectedPost] = useState(null);
    const navigate = useNavigate();

    const fetchPosts = (event) => {
        event.preventDefault();
        const queryEndDate = isSingleDay ? startDate : endDate;
        fetch(`http://localhost:8080/posts?start_date=${startDate}&end_date=${queryEndDate}`)
            .then(response => response.json())
            .then(data => {
                const formattedPosts = data.map(post => {
                    let timestamp;

                    if (post.scheduled_time && post.scheduled_time.$date) {
                        timestamp = post.scheduled_time.$date.$numberLong
                            ? parseInt(post.scheduled_time.$date.$numberLong, 10)
                            : parseInt(post.scheduled_time.$date, 10);
                    } else {
                        timestamp = parseInt(post.scheduled_time, 10);
                    }

                    const date = new Date(timestamp);
                    const formattedDate = !isNaN(date.getTime())
                        ? `${date.toLocaleDateString('en-US')} @ ${date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' })}`
                        : 'Invalid Date';

                    return {
                        ...post,
                        _id: post._id.$oid || post._id,
                        scheduled_time: formattedDate,
                    };
                });
                setPosts(formattedPosts);
            })
            .catch(error => console.error('Error fetching posts:', error));
    };

    const deletePost = (id) => {
        fetch(`http://localhost:8080/posts/${id}`, {
            method: 'DELETE',
        })
            .then(() => {
                alert('Post deleted successfully!');
                fetchPosts();
            })
            .catch(error => console.error('Error deleting post:', error));
    };

    const handleUpdateClick = (post) => {
        setSelectedPost(post);
        navigate('/update', { state: { post } });
    };

    const handleSingleDayChange = (e) => {
        setIsSingleDay(e.target.checked);
        if (e.target.checked) {
            setEndDate('');
        }
    };

    return (
        <div className="form-container post-query-container">
            <h1 className="form-title">Query Scheduled Posts</h1>
            <form onSubmit={fetchPosts} className="query-form">
                <div className="form-group date-inputs">
                    <div className="date-input">
                        <label htmlFor="startDate">Start Date</label>
                        <input
                            type="date"
                            id="startDate"
                            value={startDate}
                            onChange={(e) => setStartDate(e.target.value)}
                        />
                        <div className="single-day-option">
                            <input
                                type="checkbox"
                                id="singleDay"
                                checked={isSingleDay}
                                onChange={handleSingleDayChange}
                            />
                            <label htmlFor="singleDay">Single day</label>
                        </div>
                    </div>
                    <div className="date-input">
                        <label htmlFor="endDate">End Date</label>
                        <input
                            type="date"
                            id="endDate"
                            value={endDate}
                            onChange={(e) => setEndDate(e.target.value)}
                            disabled={isSingleDay}
                        />
                    </div>
                </div>
                <button type="submit" className="form-button search-button">Search</button>
            </form>

            <div className="results-section">
                <h2>Results</h2>
                {posts.length === 0 ? (
                    <p>No posts found for the selected date range.</p>
                ) : (
                    <ul className="post-list">
                        {posts.map(post => (
                            <li key={post._id} className="post-item">
                                <div className="post-content">
                                    <h3>{post.title}</h3>
                                    <p>{post.scheduled_time}</p>
                                </div>
                                <div className="post-actions">
                                    <button onClick={() => handleUpdateClick(post)} className="update-button">Update</button>
                                    <button onClick={() => deletePost(post._id)} className="delete-button">Delete</button>
                                </div>
                            </li>
                        ))}
                    </ul>
                )}
            </div>
        </div>
    );
};

export default PostQuery;