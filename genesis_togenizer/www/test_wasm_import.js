// Simple test to verify WASM imports work
import init, { togen_from_string } from 'genesis-togenizer';

async function test() {
  try {
    await init();
    console.log('✓ WASM initialized successfully');
    
    const result = togen_from_string('test');
    console.log('✓ togen_from_string works:', result);
    
    console.log('All tests passed!');
  } catch (error) {
    console.error('✗ Test failed:', error);
    process.exit(1);
  }
}

test();