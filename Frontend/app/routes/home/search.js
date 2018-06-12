import Route from '@ember/routing/route';
import $ from 'jqeury';

export default Route.extend({
    model(params) {
        return $.getJSON('http://localhost:8000/search/' + params.term);
    }
});