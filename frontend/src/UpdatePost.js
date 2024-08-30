import React, { useEffect, useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { Form, Button, Container, Header, Segment, Message, Loader } from 'semantic-ui-react';
import 'semantic-ui-css/semantic.min.css';

const UpdatePost = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const [post, setPost] = useState(null);
    const [title, setTitle] = useState('');
    const [content, setContent] = useState('');
    const [scheduledTime, setScheduledTime] = useState('');
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState('');

    const formatDate = (timestamp) => {
        const date = new Date(timestamp);
        const year = date.getFullYear();
        const month = String(date.getMonth() + 1).padStart(2, '0');
        const day = String(date.getDate()).padStart(2, '0');
        const hours = String(date.getHours()).padStart(2, '0');
        const minutes = String(date.getMinutes()).padStart(2, '0');
        return `${year}-${month}-${day}T${hours}:${minutes}`;
    };

    useEffect(() => {
        if (location.state?.post) {
            const postData = location.state.post;
            setPost(postData);
            setTitle(postData.title);
            setContent(postData.content);
            setScheduledTime(formatDate(postData.scheduled_time));
        } else {
            setError('No post data found. Redirecting to home page.');
            setTimeout(() => navigate('/'), 3000);
        }
        setLoading(false);
    }, [location, navigate]);

    const handleUpdate = (event) => {
        event.preventDefault();
        if (!post) return;

        setLoading(true);
        const formattedDate = scheduledTime.replace('T', ' ');

        const updatedPost = {
            title,
            content,
            scheduled_time: formattedDate,
            status: post.status,
        };

        fetch(`http://localhost:8080/posts/${post._id}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(updatedPost),
        })
            .then(response => {
                if (response.ok) {
                    setError('');
                    navigate('/', { state: { message: 'Post updated successfully!' } });
                } else {
                    return response.text().then(text => {
                        throw new Error(text);
                    });
                }
            })
            .catch(error => {
                console.error('Error updating post:', error);
                setError(`Failed to update post: ${error.message}`);
            })
            .finally(() => setLoading(false));
    };

    if (loading) return <Loader active>Loading...</Loader>;

    if (error) {
        return (
            <Container text>
                <Message negative>
                    <Message.Header>Error</Message.Header>
                    <p>{error}</p>
                </Message>
            </Container>
        );
    }

    return (
        <Container text>
            <Header as='h1' attached='top'>Update Post</Header>
            <Segment attached>
                <Form onSubmit={handleUpdate} loading={loading}>
                    <Form.Input
                        label='Title'
                        value={title}
                        onChange={(e) => setTitle(e.target.value)}
                        required
                    />
                    <Form.TextArea
                        label='Content'
                        value={content}
                        onChange={(e) => setContent(e.target.value)}
                        required
                    />
                    <Form.Input
                        label='Scheduled Time'
                        type='datetime-local'
                        value={scheduledTime}
                        onChange={(e) => setScheduledTime(e.target.value)}
                        required
                    />
                    <Button primary type='submit'>Update</Button>
                    <Button secondary onClick={() => navigate('/')}>Cancel</Button>
                </Form>
            </Segment>
        </Container>
    );
};

export default UpdatePost;