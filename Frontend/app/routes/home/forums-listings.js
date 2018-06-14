import Route from '@ember/routing/route';

export default Route.extend({
    model(params) {
        return $.getJSON("http://localhost:8000/forums/" + params.category + "/" + params.page);
    }
});
