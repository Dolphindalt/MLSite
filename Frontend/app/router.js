import EmberRouter from '@ember/routing/router';
import config from './config/environment';

const Router = EmberRouter.extend({
  location: config.locationType,
  rootURL: config.rootURL
});
 
Router.map(function() {
  this.route('register', { path: "/register/:linkuuid" });

  this.route('home', function() {
    this.route('create-post');
    this.route('view-news', { path: "/view-news/:uuid" });
    this.route('staff');
    this.route('search', { path: "/search/:term" });
    this.route('forums-listings', { path: "/forums-listings/:category/:page" });
  });
  this.route('profile', { path: "/profile/:uuid" });
  this.route('admin-panel');
  this.route('forums', function() {});
});

export default Router;
