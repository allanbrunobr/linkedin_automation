import React, { useState } from 'react';
import data from '@emoji-mart/data';
import Picker from '@emoji-mart/react';
import 'bootstrap/dist/css/bootstrap.min.css';

/**
 * A form component that allows users to schedule a post.
 *
 * The PostSchedulerForm component provides a form with fields for the post title,
 * content, and scheduled time. It also includes an emoji picker for adding emojis to the content.
 * The form data is sent to a server when the form is submitted.
 *
 * @component
 */
const PostSchedulerForm = () => {
    const [title, setTitle] = useState('');  // State for the title of the post
    const [content, setContent] = useState('');  // State for the content of the post
    const [scheduledTime, setScheduledTime] = useState('');  // State for the scheduled time of the post
    const [showEmojiPicker, setShowEmojiPicker] = useState(false);  // State to toggle the emoji picker

    /**
     * Handles form submission.
     *
     * This function is called when the form is submitted. It gathers the form data,
     * sends a POST request to the server to schedule the post, and then clears the form.
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
     * Clears the form fields.
     *
     * This function resets the form fields to their initial state.
     */
    const clearForm = () => {
        setTitle('');
        setContent('');
        setScheduledTime('');
        setShowEmojiPicker(false);
    };

    /**
     * Adds an emoji to the content.
     *
     * This function is called when an emoji is selected from the emoji picker.
     * The selected emoji is appended to the current content.
     *
     * @param {Object} emoji - The selected emoji object.
     */
    const addEmoji = (emoji) => {
        setContent(content + emoji.native);
        setShowEmojiPicker(false);
    };

    return (
        <div className="container mt-5">
            <div className="card shadow-sm">
                <div className="card-header">
                    <h2 className="h4 mb-0">Schedule a Post</h2>
                </div>
                <div className="card-body">
                    <form onSubmit={handleSubmit}>
                        <div className="mb-3">
                            <label htmlFor="title" className="form-label">Title:</label>
                            <input
                                type="text"
                                id="title"
                                value={title}
                                onChange={(e) => setTitle(e.target.value)}
                                required
                                className="form-control"
                            />
                        </div>

                        <div className="mb-3">
                            <label htmlFor="content" className="form-label">Content:</label>
                            <textarea
                                id="content"
                                value={content}
                                onChange={(e) => setContent(e.target.value)}
                                required
                                rows="4"
                                className="form-control"
                            />
                            <button
                                type="button"
                                onClick={() => setShowEmojiPicker(!showEmojiPicker)}
                                className="btn btn-outline-secondary mt-2"
                            >
                                {showEmojiPicker ? 'Close Emoji Picker' : 'Add Emoji'}
                            </button>
                            {showEmojiPicker && (
                                <Picker data={data} onEmojiSelect={addEmoji} />
                            )}
                        </div>

                        <div className="mb-3">
                            <label htmlFor="scheduledTime" className="form-label">Scheduled Time:</label>
                            <input
                                type="datetime-local"
                                id="scheduledTime"
                                value={scheduledTime}
                                onChange={(e) => setScheduledTime(e.target.value)}
                                required
                                className="form-control"
                            />
                        </div>

                        <div className="d-flex justify-content-between">
                            <button type="submit" className="btn btn-primary">
                                Schedule
                            </button>
                            <button type="button" onClick={clearForm} className="btn btn-secondary">
                                Clear
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    );
};

export default PostSchedulerForm;
