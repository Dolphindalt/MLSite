import EmberRouter from '@ember/routing/router';
import config from './config/environment';

const Router = EmberRouter.extend({
  location: config.locationType,
  rootURL: config.rootURL
});
 
Router.map(function() {
  this.route('sign-in');
  this.route('register');

  this.route('home', function() {
    this.route('create-post');
    this.route('view-news', { path: "/view-news/:uuid" });
  });
  this.route('error');
});

export default Router;
