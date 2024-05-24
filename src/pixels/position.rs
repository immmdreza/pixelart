use std::fmt::Display;

use thiserror::Error;

/// Interface for a pixel position.
pub trait PixelPositionInterface {
    /// Row number of the position starting from 0.
    fn row(&self) -> usize;

    /// Column number of the position starting from 0.
    fn column(&self) -> usize;

    /// Expand this type to a tuple of `(row, column)`.
    fn expand(&self) -> (usize, usize) {
        (self.row(), self.column())
    }

    /// Convert this [`PixelPosition`] to a [`PixelStrictPosition`].
    ///
    /// Returns [`PixelPositionOutOfBoundError`] error if provided row or column are out of bound,
    fn bound<const H: usize, const W: usize>(&self) -> StrictPositionValidationResult<H, W> {
        PixelStrictPosition::new(self.row(), self.column())
    }

    /// Returns a [`PixelPosition`] above this one as far as possible (0).
    fn up(&self, amount: usize) -> PixelPosition {
        PixelPosition::new(
            self.row().checked_sub(amount).unwrap_or_default(),
            self.column(),
        )
    }

    /// Returns a [`PixelPosition`] at the left side of this one as far as possible (0).
    fn left(&self, amount: usize) -> PixelPosition {
        PixelPosition::new(
            self.row(),
            self.column().checked_sub(amount).unwrap_or_default(),
        )
    }

    /// Returns a [`PixelPosition`] below this one as far as possible.
    fn down(&self, amount: usize) -> PixelPosition {
        PixelPosition::new(self.row().wrapping_add(amount), self.column())
    }

    /// Returns a [`PixelPosition`] at the right side of this one as far as possible.
    fn right(&self, amount: usize) -> PixelPosition {
        PixelPosition::new(self.row(), self.column().wrapping_add(amount))
    }

    /// Returns a [`PixelPosition`] at the [`Direction`] side of this one as far as possible.
    fn direction(&self, dir: Direction, amount: usize) -> PixelPosition {
        match dir {
            Direction::Up => self.up(amount),
            Direction::Right => self.right(amount),
            Direction::Down => self.down(amount),
            Direction::Left => self.left(amount),
        }
    }
}

/// Interface for a pixel position.
///
/// This type of position is limited between a H (height) and W (width).
pub trait PixelStrictPositionInterface<const H: usize, const W: usize> {
    /// Row number of the position starting from 0.
    fn row(&self) -> usize;

    /// Column number of the position starting from 0.
    fn column(&self) -> usize;

    /// Expand this type to a tuple of `(row, column)`.
    fn expand(&self) -> (usize, usize) {
        (self.row(), self.column())
    }

    /// Convert this [`PixelStrictPosition`] to a [`PixelPosition`], breaking the bounds.
    fn unbound(&self) -> PixelPosition {
        PixelPosition::new(self.row(), self.column())
    }

    /// Returns a [`PixelStrictPosition`] above this one **IF** its possible.
    fn checked_up(&self, amount: usize) -> StrictPositionValidationResult<H, W> {
        self.unbound().up(amount).bound()
    }

    /// Returns a [`PixelStrictPosition`] at the left side of this one **IF** its possible.
    fn checked_left(&self, amount: usize) -> StrictPositionValidationResult<H, W> {
        self.unbound().left(amount).bound()
    }

    /// Returns a [`PixelStrictPosition`] below this one **IF** its possible.
    fn checked_down(&self, amount: usize) -> StrictPositionValidationResult<H, W> {
        self.unbound().down(amount).bound()
    }

    /// Returns a [`PixelStrictPosition`] at the right side of this one **IF** its possible.
    fn checked_right(&self, amount: usize) -> StrictPositionValidationResult<H, W> {
        self.unbound().right(amount).bound()
    }

    /// Returns a [`PixelStrictPosition`] at the [`Direction`] side of this one **IF** its possible.
    fn checked_direction(
        &self,
        dir: Direction,
        amount: usize,
    ) -> StrictPositionValidationResult<H, W> {
        self.unbound().direction(dir, amount).bound()
    }

    /// Returns a [`PixelStrictPosition`] above this one as far as possible (0).
    fn bounding_up(&self, amount: usize) -> PixelStrictPosition<H, W> {
        self.checked_up(amount).unwrap_or_else(|e| e.adjust())
    }

    /// Returns a [`PixelStrictPosition`] at the left side of this one as far as possible (0).
    fn bounding_left(&self, amount: usize) -> PixelStrictPosition<H, W> {
        self.checked_left(amount).unwrap_or_else(|e| e.adjust())
    }

    /// Returns a [`PixelStrictPosition`] below this one as far as possible.
    fn bounding_down(&self, amount: usize) -> PixelStrictPosition<H, W> {
        self.checked_down(amount).unwrap_or_else(|e| e.adjust())
    }

    /// Returns a [`PixelStrictPosition`] at the right side of this one as far as possible.
    fn bounding_right(&self, amount: usize) -> PixelStrictPosition<H, W> {
        self.checked_right(amount).unwrap_or_else(|e| e.adjust())
    }

    /// Returns a [`PixelStrictPosition`] at the [`Direction`] side of this one as far as possible.
    fn bounding_direction(&self, dir: Direction, amount: usize) -> PixelStrictPosition<H, W> {
        self.checked_direction(dir, amount)
            .unwrap_or_else(|e| e.adjust())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PixelPosition {
    row: usize,
    column: usize,
}

impl Display for PixelPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

impl PixelPosition {
    pub const fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

impl PixelPositionInterface for PixelPosition {
    fn row(&self) -> usize {
        self.row
    }

    fn column(&self) -> usize {
        self.column
    }
}

pub type StrictPositionValidationResult<const H: usize, const W: usize> =
    Result<PixelStrictPosition<H, W>, PixelPositionOutOfBoundError<H, W>>;

#[derive(Debug, Error)]
pub enum PixelPositionOutOfBoundError<const H: usize, const W: usize> {
    #[error("The provided row value {0:?} is equal or more that row bound ({H}).")]
    InvalidRow(PixelPosition),
    #[error("The provided column value {0:?} is equal or more that column bound ({W}).")]
    InvalidColumn(PixelPosition),
    #[error("Both provided row and column values are out of bound ({H}, {W}).")]
    InvalidBoth(PixelPosition),
}

impl<const H: usize, const W: usize> PixelPositionOutOfBoundError<H, W> {
    pub fn validate_position(row: usize, column: usize) -> StrictPositionValidationResult<H, W> {
        use std::cmp::Ordering::*;
        use PixelPositionOutOfBoundError::*;

        let raw_position = PixelPosition::new(row, column);
        match (row.cmp(&H), column.cmp(&W)) {
            (Equal | Greater, Less) => Err(InvalidRow(raw_position)),
            (Less, Equal | Greater) => Err(InvalidColumn(raw_position)),
            (Equal | Greater, Equal | Greater) => Err(InvalidBoth(raw_position)),
            _ => Ok(PixelStrictPosition { raw: raw_position }),
        }
    }

    /// Adjusts the invalid position to be valid, reducing row or column to maximum allowed value.
    pub fn adjust(&self) -> PixelStrictPosition<H, W> {
        use PixelPositionOutOfBoundError::*;
        match self {
            InvalidRow(position) => PixelStrictPosition::new(H - 1, position.column).unwrap(),
            InvalidColumn(position) => PixelStrictPosition::new(position.row, W - 1).unwrap(),
            InvalidBoth(_) => PixelStrictPosition::new(H - 1, W - 1).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PixelStrictPosition<const H: usize, const W: usize> {
    raw: PixelPosition,
}

impl<const H: usize, const W: usize> Iterator for PixelStrictPosition<H, W> {
    type Item = PixelStrictPosition<H, W>;

    fn next(&mut self) -> Option<Self::Item> {
        // Going right by one to see what happens ...
        let next = self.bounding_right(1);
        if next.column() == self.column() {
            // Reached right border, should go down by one ...
            let next = self.bounding_down(1);
            if next.row() == self.row() {
                // Even moving down is not possible! nowhere no go.
                None
            } else {
                // Starting the next row from the beginning (column: 0).
                *self = PixelStrictPosition {
                    raw: PixelPosition::new(next.row(), 0),
                };
                Some(self.clone())
            }
        } else {
            // We can safely go right by one.
            *self = next.clone();
            Some(next)
        }
    }
}

impl<const H: usize, const W: usize> PixelStrictPosition<H, W> {
    /// Create a new [`PixelStrictPosition`].
    ///
    /// Returns [`PixelPositionOutOfBoundError`] error if provided row or column are out of bound,
    pub fn new(row: usize, column: usize) -> Result<Self, PixelPositionOutOfBoundError<H, W>> {
        PixelPositionOutOfBoundError::validate_position(row, column)
    }
}

impl<const H: usize, const W: usize> PixelStrictPositionInterface<H, W>
    for PixelStrictPosition<H, W>
{
    fn row(&self) -> usize {
        self.raw.row
    }

    fn column(&self) -> usize {
        self.raw.column
    }
}

impl<const H: usize, const W: usize> PixelStrictPositionInterface<H, W>
    for &PixelStrictPosition<H, W>
{
    fn row(&self) -> usize {
        self.raw.row
    }

    fn column(&self) -> usize {
        self.raw.column
    }
}

impl<const H: usize, const W: usize> PixelStrictPositionInterface<H, W>
    for &mut PixelStrictPosition<H, W>
{
    fn row(&self) -> usize {
        self.raw.row
    }

    fn column(&self) -> usize {
        self.raw.column
    }
}

pub trait IntoPixelStrictPosition<const H: usize, const W: usize> {
    fn into_pixel_strict_position(self) -> PixelStrictPosition<H, W>;
}

impl<const H: usize, const W: usize, T> IntoPixelStrictPosition<H, W> for T
where
    T: PixelStrictPositionInterface<H, W>,
{
    fn into_pixel_strict_position(self) -> PixelStrictPosition<H, W> {
        PixelStrictPosition::new(self.row(), self.column()).unwrap()
    }
}

/// A set of common useful [`PixelStrictPosition`]s inside the container
/// wrapped by square from `(H - 1, 0) -> bottom-left` to `(0, W - 1) -> top-right`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StrictPositions {
    /// The most top left side of the container `(0, 0)`.
    TopLeft,

    /// The most top right side of the container `(0, W - 1)`.
    TopRight,

    /// The most bottom right side of the container `(H - 1, W - 1)`.
    BottomRight,

    /// The most bottom left side of the container `(H - 1, 0)`.
    BottomLeft,

    /// The center of the square `(H / 2, W / 2)`.
    Center,

    /// The center of the top row `(0, W / 2)`.
    TopCenter,

    /// The center of the most right column `(H / 2, W - 1)`.
    RightCenter,

    /// The center of bottom row `(H - 1, W / 2)`.
    BottomCenter,

    /// The center of the most left column `(H / 2, 0)`.
    LeftCenter,
}

impl<const H: usize, const W: usize> PixelStrictPositionInterface<H, W> for StrictPositions {
    fn row(&self) -> usize {
        use StrictPositions::*;
        match self {
            TopLeft => 0,
            TopRight => 0,
            BottomRight => H - 1,
            BottomLeft => H - 1,
            Center => H / 2,
            TopCenter => 0,
            RightCenter => H / 2,
            BottomCenter => H - 1,
            LeftCenter => H / 2,
        }
    }

    fn column(&self) -> usize {
        use StrictPositions::*;
        match self {
            TopLeft => 0,
            TopRight => W - 1,
            BottomRight => W - 1,
            BottomLeft => 0,
            Center => W / 2,
            TopCenter => W / 2,
            RightCenter => W - 1,
            BottomCenter => W / 2,
            LeftCenter => 0,
        }
    }
}

/// Represents a direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    /// Going up.
    Up,

    /// Going right.
    Right,

    /// Going down.
    Down,

    /// Going left.
    Left,
}

impl Iterator for Direction {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
        .into()
    }
}

pub struct SingleCycle {
    initial_dir: Direction,
    current: Option<Direction>,
}

impl SingleCycle {
    pub fn new(initial_dir: Direction) -> Self {
        Self {
            initial_dir,
            current: None,
        }
    }
}

impl Iterator for SingleCycle {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut current) = self.current {
            let next = current.next().unwrap();

            if next == self.initial_dir {
                return None;
            }

            self.current = Some(next);
            self.current
        } else {
            self.current = Some(self.initial_dir);
            Some(self.initial_dir)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let pos = PixelStrictPosition::<5, 5>::new(0, 0).unwrap();

        assert_eq!(
            PixelStrictPosition {
                raw: PixelPosition::new(1, 0)
            },
            pos.bounding_down(1)
        );
        assert_eq!(
            PixelStrictPosition {
                raw: PixelPosition::new(2, 0)
            },
            pos.bounding_down(2)
        );
        assert_eq!(
            PixelStrictPosition {
                raw: PixelPosition::new(3, 0)
            },
            pos.bounding_down(3)
        );
        assert_eq!(
            PixelStrictPosition {
                raw: PixelPosition::new(4, 0)
            },
            pos.bounding_down(4)
        );
        assert_eq!(
            PixelStrictPosition {
                raw: PixelPosition::new(4, 0)
            },
            pos.bounding_down(5)
        );
        assert_eq!(
            PixelStrictPosition {
                raw: PixelPosition::new(0, 4)
            },
            pos.bounding_right(5)
        );
    }

    #[test]
    fn test_iter() {
        let mut pos = PixelStrictPosition::<2, 2>::new(0, 0).unwrap();

        assert_eq!(
            Some(PixelStrictPosition {
                raw: PixelPosition::new(0, 1)
            }),
            pos.next()
        );
        assert_eq!(
            Some(PixelStrictPosition {
                raw: PixelPosition::new(1, 0)
            }),
            pos.next()
        );
        assert_eq!(
            Some(PixelStrictPosition {
                raw: PixelPosition::new(1, 1)
            }),
            pos.next()
        );
        assert_eq!(None, pos.next());
    }

    #[test]
    fn test_direction_single_cycle() {
        use Direction::*;
        let mut single = SingleCycle::new(Down);

        assert_eq!(Some(Down), single.next());
        assert_eq!(Some(Left), single.next());
        assert_eq!(Some(Up), single.next());
        assert_eq!(Some(Right), single.next());
        assert_eq!(None, single.next());
    }
}
