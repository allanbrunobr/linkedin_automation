import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Form, Button, Checkbox, Container, Header, Segment, Table, Message, Loader, Dimmer, Modal, Icon } from 'semantic-ui-react';
import 'semantic-ui-css/semantic.min.css';
import './styles.css';

const formatDate = (milliseconds) => {
    const date = new Date(milliseconds);
    return date.toLocaleString('pt-BR', { timeZone: 'America/Sao_Paulo' });
};

const formatPostData = (post) => {
    let timestamp;
    if (post.scheduled_time?.$date) {
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
};

const PostQuery = () => {
    const [startDate, setStartDate] = useState('');
    const [endDate, setEndDate] = useState('');
    const [isSingleDay, setIsSingleDay] = useState(false);
    const [posts, setPosts] = useState([]);
    const [selectedPost, setSelectedPost] = useState(null);
    const [isLoading, setIsLoading] = useState(false);
    const [formError, setFormError] = useState('');
    const navigate = useNavigate();
    const [modalOpen, setModalOpen] = useState(false);
    const [modalMessage, setModalMessage] = useState('');
    const [postToDelete, setPostToDelete] = useState(null);
    

    const fetchPosts = () => {
        const queryEndDate = isSingleDay ? startDate : endDate;
        setIsLoading(true);

        setTimeout(() => {
            fetch(`http://localhost:8080/posts?start_date=${startDate}&end_date=${queryEndDate}`)
                .then(response => response.json())
                .then(data => {
                    const formattedPosts = data.map(formatPostData);
                    setPosts(formattedPosts);
                })
                .catch(error => console.error('Error fetching posts:', error))
                .finally(() => setIsLoading(false));
        }, 2000);
    };

    const handleFormSubmit = (event) => {
        event.preventDefault();
        setFormError('');

        if (!startDate) {
            setFormError('Start Date is required');
            return;
        }

        if (!isSingleDay && !endDate) {
            setFormError('End Date is required when not selecting a single day');
            return;
        }
        setIsLoading(true);
        fetchPosts();
    };

    const showModal = (message, post = null) => {
        setModalMessage(message);
        setPostToDelete(post);
        setModalOpen(true);
    };

    const handleDeleteConfirm = () => {
        if (postToDelete) {
            fetch(`http://localhost:8080/posts/${postToDelete._id}`, {
                method: 'DELETE',
            })
                .then(() => {
                    setModalOpen(false);
                    showModal('Post deleted successfully!');
                    fetchPosts();
                })
                .catch(error => {
                    console.error('Error deleting post:', error);
                    setModalOpen(false);
                    showModal('Error deleting post. Please try again.');
                });
        }
    };

    const handleDeleteClick = (post) => {
        showModal('Are you sure you want to delete this post?', post);
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

    const handleSingleDayChange = (e, { checked }) => {
        setIsSingleDay(checked);
        if (checked) {
            setEndDate('');
        }
        setFormError('');
    };

    const renderPostList = () => (
        <Table celled>
            <Table.Header>
                <Table.Row>
                    <Table.HeaderCell>Title</Table.HeaderCell>
                    <Table.HeaderCell>Scheduled Time</Table.HeaderCell>
                    <Table.HeaderCell>Actions</Table.HeaderCell>
                </Table.Row>
            </Table.Header>
            <Table.Body>
                {posts.map(post => (
                    <Table.Row key={post._id}>
                        <Table.Cell>{post.title}</Table.Cell>
                        <Table.Cell>{formatDate(post.scheduled_time)}</Table.Cell>
                        <Table.Cell>
                            <Button primary onClick={() => handleUpdateClick(post)}>Update</Button>
                            <Button negative onClick={() => handleDeleteClick(post)}>Delete</Button>
                        </Table.Cell>
                    </Table.Row>
                ))}
            </Table.Body>
        </Table>
    );

    const renderContent = () => {
        if (isLoading) return <Loader active>Loading</Loader>;
        if (posts.length === 0) return <Message info>No posts found for the selected date range.</Message>;
        return renderPostList();
    };

    return (
        <Container>
            <Header as='h2' attached='top'>Query Scheduled Posts</Header>
            <Segment attached>
                <Form onSubmit={handleFormSubmit} error={!!formError}>
                    <Form.Group widths='equal'>
                        <Form.Field>
                            <label>Start Date</label>
                            <input
                                type="date"
                                value={startDate}
                                onChange={(e) => setStartDate(e.target.value)}
                            />
                        </Form.Field>
                        <Form.Field>
                            <label>End Date {!isSingleDay && '*'}</label>
                            <input
                                type="date"
                                value={endDate}
                                onChange={(e) => setEndDate(e.target.value)}
                                disabled={isSingleDay}
                            />
                        </Form.Field>
                    </Form.Group>
                    <Form.Field>
                        <Checkbox
                            label='Single day'
                            checked={isSingleDay}
                            onChange={handleSingleDayChange}
                        />
                    </Form.Field>
                    {formError && <Message error content={formError} />}
                    <Button primary type='submit'>Search</Button>
                </Form>
            </Segment>

            <Segment>
                <Header as='h3'>Results</Header>
                <Dimmer.Dimmable dimmed={isLoading}>
                    <Dimmer active={isLoading} inverted>
                        <Loader>Loading</Loader>
                    </Dimmer>
                    {renderContent()}
                </Dimmer.Dimmable>
            </Segment>
            <Modal
                onClose={() => setModalOpen(false)}
                open={modalOpen}
                size='tiny'
                centered={false}
            >
                <Header icon>
                    <Icon name={postToDelete ? 'trash' : 'info circle'} />
                    {postToDelete ? 'Delete Post' : 'Information'}
                </Header>
                <Modal.Content>
                    <p>{modalMessage}</p>
                </Modal.Content>
                <Modal.Actions>
                    {postToDelete ? (
                        <>
                            <Button basic color='red' inverted onClick={() => setModalOpen(false)}>
                                <Icon name='remove' /> No
                            </Button>
                            <Button color='green' inverted onClick={handleDeleteConfirm}>
                                <Icon name='checkmark' /> Yes
                            </Button>
                        </>
                    ) : (
                        <Button color='green' inverted onClick={() => setModalOpen(false)}>
                            <Icon name='checkmark' /> OK
                        </Button>
                    )}
                </Modal.Actions>
            </Modal>
        </Container>
    );
};

export default PostQuery;