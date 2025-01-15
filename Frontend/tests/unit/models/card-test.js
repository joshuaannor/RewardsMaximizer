import { setupTest } from 'frontend/tests/helpers';
import { module, test } from 'qunit';

module('Unit | Model | card', function (hooks) {
  setupTest(hooks);

  // Replace this with your real tests.
  test('it exists', function (assert) {
    const store = this.owner.lookup('service:store');
    const model = store.createRecord('card', {});
    assert.ok(model, 'model exists');
  });
});
