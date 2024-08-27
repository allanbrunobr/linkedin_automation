import React, { useEffect, useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import './UpdatePost.css';
import './SharedStyles.css';


/**
 * UpdatePost Component - A form component that allows users to update an existing post.
 *
 * This component fetches the post data from the `location.state` (passed through React Router) and
 * populates the form fields with the post's current details. The user can modify the title, content,
 * and scheduled time of the post. Upon submission, the updated post data is sent to the server.
 *
 * @component
 * @returns {JSX.Element} The rendered UpdatePost component.
 */
const UpdatePost = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const [post, setPost] = useState(null);
    const [title, setTitle] = useState('');
    const [content, setContent] = useState('');
    const [scheduledTime, setScheduledTime] = useState('');

    /**
     * useEffect hook to initialize the form fields with the post data passed via `location.state`.
     *
     * If no post data is found in `location.state`, the user is redirected to the homepage.
     */
    useEffect(() => {
        if (location.state && location.state.post) {
            const postData = location.state.post;
            setPost(postData);
            setTitle(postData.title);
            setContent(postData.content);
            setScheduledTime(new Date(postData.scheduled_time).toISOString().slice(0, 16));
        } else {
            navigate('/');
        }
    }, [location, navigate]);

    /**
     * Handles form submission by sending the updated post data to the server.
     *
     * @param {Event} event - The form submission event.
     */
    const handleUpdate = (event) => {
        event.preventDefault();
        if (!post) return;

        const updatedPost = {
            title,
            content,
            scheduled_time: new Date(scheduledTime).toISOString(),
            status: post.status,
        };

        fetch(`http://localhost:8080/posts/${post._id}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(updatedPost),
        })
            .then(response => {
                if (response.ok) {
                    alert('Post updated successfully!');
                    navigate('/');
                } else {
                    return response.text().then(text => {
                        alert(`Failed to update post: ${text}`);
                    });
                }
            })
            .catch(error => console.error('Error updating post:', error));
    };

    if (!post) return <div>Loading...</div>;

    return (
        <div className="form-container">
            <h1 className="form-title">Update Post</h1>
            <form onSubmit={handleUpdate}>
                <div className="form-group">
                    <label htmlFor="title">Title</label>
                    <input
                        type="text"
                        id="title"
                        value={title}
                        onChange={(e) => setTitle(e.target.value)}
                        required
                    />
                </div>
                <div className="form-group">
                    <label htmlFor="content">Content</label>
                    <textarea
                        id="content"
                        value={content}
                        onChange={(e) => setContent(e.target.value)}
                        required
                    />
                </div>
                <div className="form-group">
                    <label htmlFor="scheduledTime">Scheduled Time</label>
                    <input
                        type="datetime-local"
                        id="scheduledTime"
                        value={scheduledTime}
                        onChange={(e) => setScheduledTime(e.target.value)}
                        required
                    />
                </div>
                <button type="submit" className="form-button">Update</button>
            </form>
        </div>
    );
};

export default UpdatePost;