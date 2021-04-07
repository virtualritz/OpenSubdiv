//! Table of subdivision stencils.
//!
//! Stencils are the most direct method of evaluation of locations on the limit
//! of a surface. Every point of a limit surface can be computed by linearly
//! blending a collection of coarse control vertices.
//!
//! A stencil assigns a series of control vertex indices with a blending weight
//1 that corresponds to a unique parametric location of the limit surface. When
//! the control vertices move in space, the limit location can be very
//! efficiently recomputed simply by applying the blending weights to the
//! series of coarse control vertices.
use opensubdiv_sys as sys;

use crate::Index;

use crate::far::TopologyRefiner;
pub use sys::far::stencil_table::{InterpolationMode, Options};

/// Gives read access to a single stencil in a [`StencilTable`].
pub struct Stencil<'a> {
    indices: &'a [Index],
    weights: &'a [f32],
}

impl<'a> Stencil<'a> {
    /// Returns the indices of the control vertices.
    pub fn indices(&self) -> &'a [Index] {
        self.indices
    }

    /// Returns the stencil interpolation weights.
    pub fn weights(&self) -> &'a [f32] {
        self.weights
    }
}

/// Container for stencil data.
pub struct StencilTable(pub(crate) sys::far::StencilTablePtr);

impl Drop for StencilTable {
    #[inline]
    fn drop(&mut self) {
        unsafe { sys::far::StencilTable_destroy(self.0) }
    }
}

impl StencilTable {
    /// Create a new stencil table.
    pub fn new(refiner: &TopologyRefiner, options: Options) -> StencilTable {
        let ptr =
            unsafe { sys::far::StencilTableFactory_Create(refiner.0, options) };

        if ptr.is_null() {
            panic!("StencilTableFactory_Create() returned null");
        }

        StencilTable(ptr)
    }

    /// Returns the number of stencils in the table.
    #[inline]
    pub fn len(&self) -> u32 {
        unsafe { sys::far::StencilTable_GetNumStencils(self.0) as _ }
    }

    /// Returns the number of control vertices indexed in the table.
    #[inline]
    pub fn control_vertices_len(&self) -> u32 {
        unsafe { sys::far::StencilTable_GetNumControlVertices(self.0) as _ }
    }

    /// Returns a Stencil at index i in the table.
    #[inline]
    pub fn stencil(&self, i: Index) -> Option<Stencil> {
        if self.stencils_len() <= i {
            None
        } else {
            unsafe {
                let stencil = sys::far::StencilTable_GetStencil(self.0, i);
                Some(Stencil {
                    indices: std::slice::from_raw_parts(
                        stencil._indices as _,
                        stencil._size as _,
                    ),
                    weights: std::slice::from_raw_parts(
                        stencil._weights,
                        stencil._size as _,
                    ),
                })
            }
        }
    }

    /// Returns the number of control vertices of each stencil in the table.
    #[inline]
    pub fn sizes(&self) -> &[i32] {
        unsafe {
            let vr = sys::far::StencilTable_GetSizes(self.0);
            std::slice::from_raw_parts(vr.data() as _, vr.size())
        }
    }

    /// Returns the offset to a given stencil (factory may leave empty).
    #[inline]
    pub fn offsets(&self) -> &[Index] {
        unsafe {
            let vr = sys::far::StencilTable_GetOffsets(self.0);
            std::slice::from_raw_parts(vr.data() as _, vr.size())
        }
    }

    /// Returns the indices of the control vertices.
    #[inline]
    pub fn control_indices(&self) -> &[Index] {
        unsafe {
            let vr = sys::far::StencilTable_GetControlIndices(self.0);
            std::slice::from_raw_parts(vr.data(), vr.size())
        }
    }

    /// Returns the stencil interpolation weights.
    #[inline]
    pub fn weights(&self) -> &[f32] {
        unsafe {
            let vr = sys::far::StencilTable_GetWeights(self.0);
            std::slice::from_raw_parts(vr.data(), vr.size())
        }
    }
}
