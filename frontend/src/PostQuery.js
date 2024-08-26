import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import UpdatePost from './UpdatePost';

const PostQuery = () => {
    const [startDate, setStartDate] = useState('');
    const [endDate, setEndDate] = useState('');
    const [posts, setPosts] = useState([]);
    const [selectedPost, setSelectedPost] = useState(null);
    const navigate = useNavigate();

    const fetchPosts = () => {
        fetch(`http://localhost:8080/posts?start_date=${startDate}&end_date=${endDate}`)
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

    return selectedPost ? (
        <UpdatePost post={selectedPost} />
    ) : (
        <div>
            <h2>Query Scheduled Posts</h2>
            <div className="form-group">
                <label>Start Date</label>
                <input type="date" value={startDate}
                       onChange={e => setStartDate(e.target.value)}
                       className="form-control" />
            </div>
            <div className="form-group">
                <label>End Date</label>
                <input type="date" value={endDate}
                       onChange={e => setEndDate(e.target.value)}
                       className="form-control" />
            </div>
            <button onClick={fetchPosts} className="btn btn-primary">Query</button>

            <h3>Results</h3>
            <ul>
                {posts.map(post => (
                    <li key={post._id}>
                        <strong>{post.title}</strong> - {post.scheduled_time}
                        <button onClick={() => deletePost(post._id)}
                                className="btn btn-danger btn-sm">Delete</button>
                        <button onClick={() => handleUpdateClick(post)}
                                className="btn btn-warning btn-sm">Update</button>
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default PostQuery;
