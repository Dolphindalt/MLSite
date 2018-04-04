import Route from '@ember/routing/route';
import { v4 } from 'ember-uuid'; 
import { inject as service } from '@ember/service';

export default Route.extend({
    session: service('auth-service'),
    currentDate: service('current-date'),
    admin: service('admin-priv'),
    date: service('current-date'),
    errorMessage: "",
    beforeModel() {
        this.get('admin').adminRoute(this);
    },
    model() {
        return this.store.createRecord('news-post-model');
    },
    actions: {
        post_news() {
            console.debug("K");
            let new_post = this.modelFor('home.create-post');
            new_post.set('author', this.get('session').get('username'));
            new_post.set('datetime', this.get('date').getDate());
            new_post.set('uuid', v4());
            let route = this;
            new_post.save().then(function() {
                console.debug("WRRRYYY");
                route.transitionTo('index');
            }).catch(function(reason) {
                route.set('errorMessage', reason);
                console.debug("Errr");
            });
        }
    }
});
