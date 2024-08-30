import React, { useEffect, useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { Form, Button, Container, Header, Segment, Message, Loader, Icon, Grid, Modal } from 'semantic-ui-react';
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
    const [modalOpen, setModalOpen] = useState(false);
    const [modalMessage, setModalMessage] = useState('');
    const [modalSuccess, setModalSuccess] = useState(true);

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
            showModal('No post data found. Redirecting to home page.', false);
             setTimeout(() => navigate('/'), 3000);
        }
        setLoading(false);
    }, [location, navigate]);

    const showModal = (message, success = true) => {
        setModalMessage(message);
        setModalSuccess(success);
        setModalOpen(true);
    };

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
                    showModal('Post updated successfully!');
                    //navigate('/', { state: { message: 'Post updated successfully!' } });
                } else {
                    return response.text().then(text => {
                        throw new Error(text);
                    });
                }
            })
            .catch(error => {
                console.error('Error updating post:', error);
                showModal(`Failed to update post: ${error.message}`, false);
            })
            .finally(() => setLoading(false));
    };
    const handleModalClose = () => {
        setModalOpen(false);
        if (modalSuccess) {
            navigate('/');
        }
    };

    if (loading) return <Loader active>Loading...</Loader>;

    if (error) {
        return (
            <Container>
                <Message negative>
                    <Message.Header>Error</Message.Header>
                    <p>{error}</p>
                </Message>
            </Container>
        );
    }

    return (
        <Container>
            <Segment padded="very">
                <Header as="h2" icon textAlign="center">
                    <Icon name="edit" circular />
                    <Header.Content>Update Post</Header.Content>
                </Header>
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
                        label='Scheduled Time (Brazil Time - UTC-3)'
                        type='datetime-local'
                        value={scheduledTime}
                        onChange={(e) => setScheduledTime(e.target.value)}
                        required
                    />
                    <Grid>
                        <Grid.Column textAlign="right">
                            <Button.Group>
                                <Button secondary onClick={() => navigate('/')}>Cancel</Button>
                                <Button.Or />
                                <Button positive type='submit'>Update</Button>
                            </Button.Group>
                        </Grid.Column>
                    </Grid>
                </Form>
            </Segment>
            <Modal
                onClose={handleModalClose}
                open={modalOpen}
                size='tiny'
                centered={false}
            >
                <Header icon>
                    <Icon name={modalSuccess ? 'check circle' : 'exclamation triangle'} />
                    {modalSuccess ? 'Success' : 'Error'}
                </Header>
                <Modal.Content>
                    <p>{modalMessage}</p>
                </Modal.Content>
                <Modal.Actions>
                    <Button color={modalSuccess ? 'green' : 'red'} inverted onClick={handleModalClose}>
                        <Icon name='checkmark' /> OK
                    </Button>
                </Modal.Actions>
            </Modal>
        </Container>
    );
};

export default UpdatePost;