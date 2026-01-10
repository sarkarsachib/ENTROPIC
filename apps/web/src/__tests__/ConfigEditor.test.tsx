import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { BrowserRouter } from 'react-router-dom';
import ConfigEditor from '../pages/ConfigEditor';

describe('ConfigEditor', () => {
  it('should validate config in real-time when changing camera', async () => {
    render(
      <BrowserRouter>
        <ConfigEditor />
      </BrowserRouter>
    );

    // Find camera select
    const cameraSelect = await screen.findByDisplayValue('2D Perspective');
    fireEvent.change(cameraSelect, { target: { value: 'Isometric' } });

    await waitFor(() => {
      // Validation section should exist
      expect(screen.getByText('Validation')).toBeInTheDocument();
    });
  });
});
