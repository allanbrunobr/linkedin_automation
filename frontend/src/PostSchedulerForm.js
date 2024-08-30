import React, { useState } from 'react';
import data from '@emoji-mart/data';
import Picker from '@emoji-mart/react';
import { Form, Input, TextArea, Button, Header, Icon, Segment, Grid, Popup } from 'semantic-ui-react';
import 'semantic-ui-css/semantic.min.css';
import './styles.css';

/**
 * PostSchedulerForm Component - A form component that allows users to schedule a post.
 *
 * This component provides a form with fields for the post title, content, and scheduled time.
 * It also includes an emoji picker for adding emojis to the content. The form data is sent to a server
 * when the form is submitted.
 *
 * @component
 * @returns {JSX.Element} The rendered PostSchedulerForm component.
 */
const PostSchedulerForm = () => {
    const [title, setTitle] = useState('');
    const [content, setContent] = useState('');
    const [scheduledTime, setScheduledTime] = useState('');
    const [showEmojiPicker, setShowEmojiPicker] = useState(false);
    const [loading, setLoading] = useState(false);

    /**
     * Handles form submission by sending the post data to the server.
     *
     * @param {Event} event - The form submission event.
     */
    const handleSubmit = (event) => {
        event.preventDefault();
        setLoading(true);
        const formattedDate = scheduledTime.replace('T', ' ');

        const post = {
            title,
            content,
            scheduled_time: formattedDate,
            status: 'pending',
        };

        fetch('http://localhost:8080/schedule', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(post),
        })
            .then(response => response.text())
            .then(data => {
                setLoading(false);
                clearForm();
                alert('Post scheduled successfully!');
            })
            .catch((error) => {
                setLoading(false);
                console.error('Error:', error);
            });
    };

    /**
     * Clears the form fields, resetting the form to its initial state.
     */
    const clearForm = () => {
        setTitle('');
        setContent('');
        setScheduledTime('');
        setShowEmojiPicker(false);
    };

    /**
     * Adds an emoji to the post content.
     *
     * @param {Object} emoji - The selected emoji object.
     */
    const addEmoji = (emoji) => {
        setContent(content + emoji.native);
        setShowEmojiPicker(false);
    };

    return (
        <Segment padded="very">
            <Header as="h2" icon textAlign="center">
                <Icon name="calendar plus" circular />
                <Header.Content>Schedule a Post</Header.Content>
            </Header>
            <Form onSubmit={handleSubmit} loading={loading}>
                <Form.Field
                    control={Input}
                    label="Title"
                    placeholder="Enter post title"
                    value={title}
                    onChange={(e) => setTitle(e.target.value)}
                    required
                />
                <Form.Field
                    control={TextArea}
                    label="Content"
                    placeholder="Enter post content"
                    value={content}
                    onChange={(e) => setContent(e.target.value)}
                    required
                />
                <Popup
                    trigger={
                        <Button
                            icon="smile outline"
                            content="Add Emoji"
                            type="button"
                            onClick={() => setShowEmojiPicker(!showEmojiPicker)}
                        />
                    }
                    content={showEmojiPicker && <Picker data={data} onEmojiSelect={addEmoji} />}
                    on="click"
                    position="bottom left"
                />
                <Form.Field
                    control={Input}
                    label="Scheduled Time (Brazil Time - UTC-3)"
                    type="datetime-local"
                    value={scheduledTime}
                    onChange={(e) => setScheduledTime(e.target.value)}
                    required
                />
                <Grid>
                    <Grid.Column textAlign="right">
                        <Button.Group>
                            <Button type="button" onClick={clearForm}>Clear</Button>
                            <Button.Or />
                            <Button positive type="submit">Schedule</Button>
                        </Button.Group>
                    </Grid.Column>
                </Grid>
            </Form>
        </Segment>
    );
};

export default PostSchedulerForm;