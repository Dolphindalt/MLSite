import ApplicationAdapter from './application';
import $ from 'jquery';

export default ApplicationAdapter.extend({
    host: 'http://localhost:8000',
    queryRecord(store, type, query) {
        return $.getJSON("/news-post-models/" + query.uuid);
    }
});