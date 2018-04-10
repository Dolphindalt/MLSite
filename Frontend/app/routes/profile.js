import Route from '@ember/routing/route';
import $ from 'jquery';

export default Route.extend({
    model(params) {
        return $.getJSON('http://localhost:8000/user/' + params.username);
    }
});
