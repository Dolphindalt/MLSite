import Route from '@ember/routing/route';

export default Route.extend({
    model(params) {
        return data = $.getJSON('http://localhost:8000/search/' + params.term);
    }
});