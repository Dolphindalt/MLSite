import Route from '@ember/routing/route';
import $ from 'jquery';

export default Route.extend({
    model() {
        return $.ajax({
            type: "GET",
            dataType: 'json',
            url: "http://127.0.0.1:8000/news-post-models",
            crossDomain: true,
            contentType: "application/json; charset=utf-8"
        });
    }
});
