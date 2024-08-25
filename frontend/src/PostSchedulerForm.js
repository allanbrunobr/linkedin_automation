import React, { useState } from 'react';
import data from '@emoji-mart/data';
import Picker from '@emoji-mart/react';
import 'bootstrap/dist/css/bootstrap.min.css';

const PostSchedulerForm = () => {
    const [title, setTitle] = useState('');
    const [content, setContent] = useState('');
    const [scheduledTime, setScheduledTime] = useState('');
    const [showEmojiPicker, setShowEmojiPicker] = useState(false);

    const handleSubmit = (event) => {
        event.preventDefault();
        const post = {
            title,
            content,
            scheduled_time: new Date(scheduledTime).toISOString(),
            status: 'pending',
        };

        fetch('http://localhost:8080/schedule', {
            method: 'POST', // Especificando o método HTTP correto
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

    const clearForm = () => {
        setTitle('');
        setContent('');
        setScheduledTime('');
        setShowEmojiPicker(false);
    };

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
