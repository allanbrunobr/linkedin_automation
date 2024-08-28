import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import './styles.css';

/**
 * PostQuery Component - This component provides an interface for querying scheduled posts
 * based on a date range. Users can search for posts, update them, or delete them.
 *
 * @component
 * @returns {JSX.Element} The rendered component.
 */
const PostQuery = () => {
    const [startDate, setStartDate] = useState('');
    const [endDate, setEndDate] = useState('');
    const [isSingleDay, setIsSingleDay] = useState(false);
    const [posts, setPosts] = useState([]);
    const [selectedPost, setSelectedPost] = useState(null);
    const navigate = useNavigate();

    const formatDate = (milliseconds) => {
        const date = new Date(milliseconds);
        return date.toLocaleString('pt-BR', { timeZone: 'America/Sao_Paulo' });
    };
    /**
     * Fetches posts from the backend based on the selected date range.
     * The posts are formatted before being stored in the state.
     */
    const fetchPosts = () => {
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

    /**
     * Handles form submission to fetch posts based on the date range.
     *
     * @param {Event} event - The form submission event.
     */
    const handleFormSubmit = (event) => {
        event.preventDefault();
        fetchPosts();
    };

    /**
     * Deletes a post based on its ID and refetches the updated list of posts.
     *
     * @param {string} id - The ID of the post to be deleted.
     */
    const deletePost = (id) => {
        fetch(`http://localhost:8080/posts/${id}`, {
            method: 'DELETE',
        })
            .then(() => {
                alert('Post deleted successfully!');
                fetchPosts();  // Re-fetch posts after deletion
            })
            .catch(error => console.error('Error deleting post:', error));
    };

    /**
     * Navigates to the update screen with the selected post's details.
     *
     * @param {Object} post - The post object to be updated.
     */
    const handleUpdateClick = (post) => {
        setSelectedPost(post);
        navigate('/update', { state: { post } });
    };

    /**
     * Handles the change of the "Single Day" checkbox.
     * If checked, clears the end date.
     *
     * @param {Event} e - The change event.
     */
    const handleSingleDayChange = (e) => {
        setIsSingleDay(e.target.checked);
        if (e.target.checked) {
            setEndDate('');
        }
    };

    return (
        <div className="form-container">
            <h2 className="form-title">Query Scheduled Posts</h2>
            <form onSubmit={handleFormSubmit} className="query-form">
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
                                    <p>{formatDate(post.scheduled_time)}</p>
                                </div>
                                <div className="post-actions">
                                    <button onClick={() => handleUpdateClick(post)}
                                            className="form-button update-button">Update
                                    </button>
                                    <button onClick={() => deletePost(post._id)}
                                            className="form-button delete-button">Delete
                                    </button>
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
