import React, { useState } from 'react';
import data from '@emoji-mart/data';
import Picker from '@emoji-mart/react';
import 'bootstrap/dist/css/bootstrap.min.css';
import './SharedStyles.css';
import './PostSchedulerForm.css';

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

    /**
     * Handles form submission by sending the post data to the server.
     *
     * @param {Event} event - The form submission event.
     */
    const handleSubmit = (event) => {
        event.preventDefault();
        const post = {
            title,
            content,
            scheduled_time: new Date(scheduledTime).toISOString(),
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
                alert('Post scheduled successfully!');
                clearForm();
            })
            .catch((error) => {
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
        <div className="form-container">
            <h2 className="form-title">Schedule a Post</h2>
            <form onSubmit={handleSubmit}>
                <div className="form-group">
                    <label htmlFor="title">Title:</label>
                    <input
                        type="text"
                        id="title"
                        value={title}
                        onChange={(e) => setTitle(e.target.value)}
                        required
                    />
                </div>

                <div className="form-group">
                    <label htmlFor="content">Content:</label>
                    <textarea
                        id="content"
                        value={content}
                        onChange={(e) => setContent(e.target.value)}
                        required
                        rows="5"
                    />
                    <button
                        type="button"
                        onClick={() => setShowEmojiPicker(!showEmojiPicker)}
                        className="form-button"
                    >
                        {showEmojiPicker ? 'Close Emoji Picker' : 'Add Emoji'}
                    </button>
                    {showEmojiPicker && (
                        <Picker data={data} onEmojiSelect={addEmoji} />
                    )}
                </div>

                <div className="form-group">
                    <label htmlFor="scheduledTime">Scheduled Time:</label>
                    <input
                        type="datetime-local"
                        id="scheduledTime"
                        value={scheduledTime}
                        onChange={(e) => setScheduledTime(e.target.value)}
                        required
                    />
                </div>

                <div className="button-group">
                    <button type="submit" className="form-button">Schedule</button>
                    <button type="button" onClick={clearForm} className="form-button">Clear</button>
                </div>
            </form>
        </div>
    );
};

export default PostSchedulerForm;