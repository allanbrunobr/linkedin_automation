import React, { useEffect, useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';

const UpdatePost = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const [post, setPost] = useState(null);
    const [title, setTitle] = useState('');
    const [content, setContent] = useState('');
    const [scheduledTime, setScheduledTime] = useState('');

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
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(updatedPost),
        })
            .then(response => {
                if (response.ok) {
                    alert('Post updated successfully!');
                    navigate('/'); // Navigate back to the list view after update
                } else {
                    return response.text().then(text => {
                        alert(`Failed to update post: ${text}`);
                    });
                }
            })
            .catch(error => console.error('Error updating post:', error));
    };

    if (!post) return <div>Loading...</div>; // Add a loading state

    return (
        <div>
            <h2>Update Post</h2>
            <form onSubmit={handleUpdate}>
                <div className="form-group">
                    <label>Title</label>
                    <input
                        type="text"
                        value={title}
                        onChange={(e) => setTitle(e.target.value)}
                        className="form-control"
                        required
                    />
                </div>
                <div className="form-group">
                    <label>Content</label>
                    <textarea
                        value={content}
                        onChange={(e) => setContent(e.target.value)}
                        className="form-control"
                        required
                    />
                </div>
                <div className="form-group">
                    <label>Scheduled Time</label>
                    <input
                        type="datetime-local"
                        value={scheduledTime}
                        onChange={(e) => setScheduledTime(e.target.value)}
                        className="form-control"
                        required
                    />
                </div>
                <button type="submit" className="btn btn-primary">Update</button>
            </form>
        </div>
    );
};

export default UpdatePost;